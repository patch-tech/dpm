from datetime import datetime
from typing import Dict, List, Literal, Union

from python_graphql_client import GraphqlClient

from ..field import DateField, DateTimeField, DerivedField, LiteralField
from ..field_expr import AggregateFieldExpr, Operator, UnaryBooleanFieldExpr
from .interface import Backend

PatchOperator = Union[Operator, Literal["before"], Literal["after"]]


def snake_to_camel(snake_str):
    parts = snake_str.split("_")
    first = parts[0].lower()
    rest = "".join([word.capitalize() for word in parts[1:]])
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
    elif isinstance(field, (AggregateFieldExpr, DerivedField)):
        base_field = field.operands[0]
        base_field_gql = field_as_graphql(
            base_field, False
        )  # Don't alias the base field.
        field_name = snake_to_camel(f"{base_field_gql}_{field.operator}")
        return with_alias(field_name, field.alias) if use_alias else field_name
    elif field.operator() != "ident":
        raise ValueError(f'Unexpected field expression "{field}"')
    else:
        field_name = snake_to_camel(field.operands()[0])
        return with_alias(field_name, field.alias) if use_alias else field_name


def selection_as_graphql(selection):
    gql_fragments = []
    for field_expr in selection:
        try:
            gql_fragment = field_as_graphql(
                field_expr, True
            )  # Use alias in selection set.
            gql_fragments.append(gql_fragment)
        except Exception as e:
            # Unexpected selection fieldExpr.
            raise Exception(
                f'Unexpected selection field expression "{field_expr}"'
            ) from e
    return "\n".join(gql_fragments)


def format_default(op, lhs, rhs):
    lhs_gql = field_as_graphql(lhs)
    rhs_gql = field_as_graphql(rhs)
    return f"""{{
    {lhs_gql}: {{
      {op}: {rhs_gql}
    }}
  }}"""

def format_unary(op, operand, _rhs):
    operand_gql = field_as_graphql(operand)
    return f"""{{
    {operand_gql}: {{
      {op}: null
    }}
  }}"""


def format_in_past(_op, lhs, rhs):
    if not isinstance(rhs.value, list) or len(rhs.value) != 3:
        raise ValueError(
            f"Patch error: inPast specified with invalid arguments {rhs.value}, must have [<number>, <number>, <granularity>]"
        )
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
    if op == "lt" or op == "lte":
        temporal_op = "before"
    elif op == "gt" or op == "gte":
        temporal_op = "after"

    if temporal_op == "eq" or temporal_op == "neq":
        raise ValueError(
            f"Patch error: {temporal_op} not supported for temporal fields"
        )

    return format_default(temporal_op, lhs, rhs)


def get_op_formatter(op, lhs):
    if op == "inPast":
        return format_in_past

    if op == "isNull" or op == "isNotNull":
        return format_unary

    if isinstance(lhs, (DateField, DateTimeField)):
        return format_temporal

    return format_default


def expr_as_graphql(expr):
    op = expr.operator()
    newline = ",\n"
    if op == "and" or op == "or":
        operands_gql = [expr_as_graphql(e) for e in expr.operands()]
        return f"""{{
      {op}: [{newline.join(operands_gql)}]
    }}"""
    lhs, rhs = expr.operands()

    #  UnaryBooleanFieldExpr has no RHS, so create one to appease the isinstance check below.
    # The unary operator formatter ignores the rhs anyway.
    if isinstance(expr, UnaryBooleanFieldExpr):
        rhs = LiteralField(True)

    if not isinstance(rhs, LiteralField):
        raise ValueError(
            f"Patch error: non-literal RHS not supported in expression: {lhs.to_string()} {op} {rhs.to_string()}"
        )
    formatter = get_op_formatter(op, lhs)
    return formatter(op, lhs, rhs)


def query_name_as_graphql(name):
    return f"{snake_to_camel(name)}Query"


class Patch(Backend):
    def __init__(self, path: str, dataset_name: str, version: str, auth_token: str):
        self.path = path
        self.dataset_name = dataset_name
        self.version = version
        self.auth_token = auth_token

    async def compile(self, query) -> str:
        query_name = query_name_as_graphql(query.name)
        filter_expr = query.filter_expr
        selection = query.selection
        order_by = query.ordering
        limit_to = query.limit_to

        if not selection:
            raise ValueError("Queries to patch must include a selection")

        selection_fragment = selection_as_graphql(selection)

        param_parts: List[str] = []
        if filter_expr:
            param_parts.append(f"filter: {expr_as_graphql(filter_expr)}")

        if order_by:
            ordering = ", ".join(
                [
                    f"{{{field_as_graphql(field)}: {dir.lower()}}}"
                    for field, dir in order_by
                ]
            )
            param_parts.append(f"orderBy: [{ordering}]")

        param_parts.append(f"limit: {limit_to}")

        compiledQuery = (
            f"{query_name}({', '.join(param_parts)}) {{\n{selection_fragment}\n}}"
        )
        return compiledQuery

    async def execute(self, query) -> List[Dict]:
        source_path = query.source
        if not source_path:
            raise TypeError(
                "Cannot execute query whose table does not have a source specified"
            )

        compiled_query = await self.compile(query)

        graphQLClient = GraphqlClient(
            source_path, headers={"authorization": f"Bearer {self.auth_token}"}
        )

        data = graphQLClient.execute(f"{{{compiled_query}}}")["data"]

        query_name = query_name_as_graphql(query.name)
        return data[query_name]
