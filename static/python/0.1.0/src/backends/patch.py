from typing import List, Dict, Union, Literal
from datetime import datetime
from field import DateField, DateTimeField, DerivedField, LiteralField
from fieldExpr import (
    AggregateFieldExpr,
    BooleanFieldExpr,
    DateTimeGranularity,
    FieldExpr,
    Operator,
    Scalar,
)
from table import Table
from interface import Backend
from graphqlclient import GraphQLClient

PatchOperator = Union[Operator, Literal["before"], Literal["after"]]

def snake_to_camel(snake_str):
    parts = snake_str.split('_')
    first = parts[0]
    rest = ''.join([word.capitalize() for word in parts[1:]])
    return first + rest

def with_alias(field_name, alias=None):
    if alias:
        return f"{alias}: {field_name}"
    else:
        return field_name
    
def field_as_graphql(field, use_alias=False):
    def stringify(x):
        return f'"{x}"' if isinstance(x, (str, datetime.date)) else str(x)

    if isinstance(field, LiteralField):
        if isinstance(field.value, list):
            return f'[{", ".join(stringify(x) for x in field.value)}]'
        else:
            return stringify(field.value)
    elif isinstance(field, AggregateFieldExpr) or isinstance(field, DerivedField):
        base_field = field.operands[0]
        base_field_gql = field_as_graphql(base_field, False) # Don't alias the base field.
        field_name = snake_to_camel(f'{base_field_gql}_{field.operator}')
        return with_alias(field_name, field.alias) if use_alias else field_name
    elif field.operator != 'ident':
        raise ValueError(f'Unexpected field expression {field}')
    else:
        field_name = snake_to_camel(field.operands[0])
        return with_alias(field_name, field.alias) if use_alias else field_name
    
def selection_as_graphql(selection):
    gql_fragments = []
    for field_expr in selection:
        try:
            gql_fragment = field_as_graphql(field_expr, True) # Use alias in selection set.
            gql_fragments.append(gql_fragment)
        except Exception as e:
            # Unexpected selection fieldExpr.
            raise Exception(f"Unexpected selection field expression {field_expr}") from e
    return '\n'.join(gql_fragments)

def format_default(op, lhs, rhs):
    lhs_gql = field_as_graphql(lhs)
    rhs_gql = field_as_graphql(rhs)
    return f"""{{
    {lhs_gql}: {{
      {op}: {rhs_gql}
    }}
  }}"""

def format_in_past(_op, lhs, rhs):
    if not isinstance(rhs.value, list) or len(rhs.value) != 3:
        raise ValueError(f"Patch error: inPast specified with invalid arguments {rhs.value}, must have [<number>, <number>, <granularity>]")
    older_than, newer_than, granularity = rhs.value
    lhs_gql = field_as_graphql(lhs)
    return f"""{{
    {lhs_gql}: {{
      olderThan: {{{granularity}: {older_than}}},
      newerThan: {{{granularity}: {newer_than}}},
    }}
  }}"""

def format_temporal(op, lhs, rhs):
    temporal_op = op
    if op == 'lt' or op == 'lte':
        temporal_op = 'before'
    elif op == 'gt' or op == 'gte':
        temporal_op = 'after'

    if temporal_op == 'eq' or temporal_op == 'neq':
        raise ValueError(f"Patch error: {temporal_op} not supported for temporal fields")

    return format_default(temporal_op, lhs, rhs)

def get_op_formatter(op, lhs):
    if op == 'inPast':
        return format_in_past

    if isinstance(lhs, (DateField, DateTimeField)):
        return format_temporal

    return format_default

def expr_as_graphql(expr):
    op = expr.operator()
    newline = ',\n'
    if op == 'and' or op == 'or':
        operands_gql = [expr_as_graphql(e) for e in expr.operands()]
        return f"""{{
      {op}: [{newline.join(operands_gql)}]
    }}"""
    lhs, rhs = expr.operands()
    if not isinstance(rhs, LiteralField):
        raise ValueError(f"Patch error: non-literal RHS not supported in expression: {lhs.toString()} {op} {rhs.toString()}")
    formatter = get_op_formatter(op, lhs)
    return formatter(op, lhs, rhs)

def query_name_as_graphql(name):
    return f"{snake_to_camel(name)}Query"

class Patch:
    def __init__(self, path: str, datasetName: str, version: str, authToken: str):
        self.path = path
        self.datasetName = datasetName
        self.version = version
        self.authToken = authToken
    
    async def compile(self, query: Table) -> str:
        queryName = query_name_as_graphql(query.name)
        filterExpr = query.filterExpr
        selection = query.selection
        orderBy = query.ordering
        limitTo = query.limitTo
        
        if not selection:
            raise ValueError('Queries to patch must include a selection')
        
        selectionFragment = selection_as_graphql(selection)
        
        paramParts: List[str] = []
        if filterExpr:
            paramParts.append(f"filter: {expr_as_graphql(filterExpr)}")
        
        if orderBy:
            ordering = ", ".join([f"{{{field_as_graphql(field)}: {dir.lower()}}}" for field, dir in orderBy])
            paramParts.append(f"orderBy: [{ordering}]")
        
        paramParts.append(f"limit: {limitTo}")
        
        compiledQuery = f"{queryName}({', '.join(paramParts)}) {{\n{selectionFragment}\n}}"
        return compiledQuery

    async def execute(self, query: Table) -> List[Dict[str, Union[int, str, bool]]]:
        sourcePath = query.source
        if not sourcePath:
            raise TypeError('Cannot execute query whose table does not have a source specified')
        
        compiledQuery = await self.compile(query)
        
        graphQLClient = GraphQLClient(sourcePath, headers={"authorization": f"Bearer {self.authToken}"})
        
        data = graphQLClient.execute_query(f"{{{compiledQuery}}}")
        
        queryName = query_name_as_graphql(query.name)
        return data[queryName]