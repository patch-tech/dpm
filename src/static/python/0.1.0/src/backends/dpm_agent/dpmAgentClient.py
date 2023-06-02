from typing import List, Tuple
import json
from grpc import ChannelCredentials, ServiceError
from dpm_agent_pb import (
    AggregateExpression, BooleanExpression, CompiledQuery, ConnectionRequest,
    ConnectionResponse, DerivedExpression, DpmAgentQuery, FieldReference, GroupByExpression,
    Literal, OrderByExpression, QueryResult, Scalar, SelectExpression
)
from dpm_agent_grpc_pb import DpmAgentClient as DpmAgentGrpcClient
from field import (
    AggregateFieldExpr, BooleanFieldExpr, DerivedField, FieldExpr, LiteralField,
    ProjectionOperator, Scalar
)
from table import Ordering, Table


def make_dpm_literal(literal: LiteralField[Scalar]) -> Literal:
    def make_literal(x: Scalar) -> Literal:
        dpm_lit = Literal()
        if isinstance(x, str):
            return dpm_lit.set_string(x)
        elif isinstance(x, int):
            return dpm_lit.set_i64(x) if isinstance(x, int) else dpm_lit.set_f64(x)
        elif isinstance(x, bool):
            return dpm_lit.set_boolean(x)
        return dpm_lit.set_timestammp(int(x))

    if isinstance(literal.value, list):
        return Literal().set_list(
            Literal.List().set_values_list([make_literal(val) for val in literal.value])
        )
    return make_literal(literal.value)


def make_dpm_field_reference(field: FieldExpr) -> FieldReference:
    return FieldReference().set_fieldname(field.operands()[0].to_string())


AGGREGATE_OPERATOR_MAP = {
    'min': AggregateExpression.MIN,
    'max': AggregateExpression.MAX,
    'count': AggregateExpression.COUNT,
    'countDistinct': AggregateExpression.COUNT_DISTINCT,
    'avg': AggregateExpression.MEAN,
    'avgDistinct': AggregateExpression.MEAN,
}


def make_dpm_aggregate_expression(agg_expr: AggregateFieldExpr[Scalar]) -> AggregateExpression:
    base_field = agg_expr.operands()[0]
    base_dpm_expr = make_dpm_expression(base_field)
    agg_op = agg_expr.operator()
    dpm_agg_op = AGGREGATE_OPERATOR_MAP.get(agg_op)
    if dpm_agg_op is None:
        raise ValueError(f'Unsupported aggregate operation {agg_op}')

    return AggregateExpression().set_argument(base_dpm_expr).set_op(dpm_agg_op)


PROJECTION_OPERATOR_MAP = {
    'day': DerivedExpression.DAY,
    'month': DerivedExpression.MONTH,
    'year': DerivedExpression.YEAR,
    'hour': DerivedExpression.HOUR,
    'minute': DerivedExpression.MINUTE,
    'second': DerivedExpression.SECOND,
    'millisecond': DerivedExpression.MILLISECOND,
}


def make_dpm_derived_expression(derived_field: DerivedField[Scalar, Scalar]) -> DerivedExpression:
    base_field = derived_field.operands()[0]
    base_dpm_expr = make_dpm_expression(base_field)
    projection_op = derived_field.operator()
    dpm_projection_op = PROJECTION_OPERATOR_MAP.get(projection_op)
    if dpm_projection_op is None:
        raise ValueError(f'Unsupported projection operation {projection_op}')

    return DerivedExpression().set_argument(base_dpm_expr).set_op(dpm_projection_op)


def make_dpm_expression(field: FieldExpr) -> DpmAgentQuery.Expression:
    if isinstance(field, LiteralField):
        return DpmAgentQuery.Expression().set_literal(make_dpm_literal(field))
    elif isinstance(field, AggregateFieldExpr):
        return DpmAgentQuery.Expression().set_aggregate(make_dpm_aggregate_expression(field))
    elif isinstance(field, DerivedField):
        return DpmAgentQuery.Expression().set_derived(make_dpm_derived_expression(field))
    elif field.operator() != 'ident':
        raise ValueError(f'Unexpected field expression {field}')
    return DpmAgentQuery.Expression().set_field(make_dpm_field_reference(field))


def make_dpm_group_by_expression(field: FieldExpr) -> GroupByExpression:
    if isinstance(field, DerivedField):
        return GroupByExpression().set_derived(make_dpm_derived_expression(field))
    elif field.operator() != 'ident':
        raise ValueError(f'Unexpected field expression in groupBy: {field}')
    return GroupByExpression().set_field(make_dpm_field_reference(field))


def make_dpm_select_expression(field: FieldExpr) -> SelectExpression:
    select_expr = SelectExpression().set_argument(make_dpm_expression(field))
    if field.alias is not None:
        return select_expr.set_alias(field.alias)
    return select_expr

booleanOperatorMap = {
    'and': DpmAgentQuery.BooleanExpression.BooleanOperator.AND,
    'or': DpmAgentQuery.BooleanExpression.BooleanOperator.OR,
    'eq': DpmAgentQuery.BooleanExpression.BooleanOperator.EQ,
    'neq': DpmAgentQuery.BooleanExpression.BooleanOperator.NEQ,
    'gt': DpmAgentQuery.BooleanExpression.BooleanOperator.GT,
    'gte': DpmAgentQuery.BooleanExpression.BooleanOperator.GTE,
    'lt': DpmAgentQuery.BooleanExpression.BooleanOperator.LT,
    'lte': DpmAgentQuery.BooleanExpression.BooleanOperator.LTE,
    'like': DpmAgentQuery.BooleanExpression.BooleanOperator.LIKE,
    'between': DpmAgentQuery.BooleanExpression.BooleanOperator.BETWEEN,
    'in': DpmAgentQuery.BooleanExpression.BooleanOperator.IN,
    # todo(PAT-3175, PAT-3176): Define once we support unary not.
    'not': None,
    # todo(PAT-3355): Remove `inPast` once we redefine it in terms of a `between` check.
    'inPast': None,
}

def make_dpm_boolean_expression(filter: BooleanFieldExpr) -> DpmAgentQuery.BooleanExpression:
    op = filter.operator()
    if op == 'and' or op == 'or':
        args = [DpmAgentQuery.Expression().set_condition(make_dpm_boolean_expression(expr)) for expr in filter.operands()]
        return DpmAgentQuery.BooleanExpression().set_op(booleanOperatorMap[op]).set_arguments_list(args)

    dpm_boolean_op = booleanOperatorMap[op]
    if dpm_boolean_op is None:
        raise ValueError(f'Unhandled boolean operator {op}')

    args = [make_dpm_expression(expr) for expr in filter.operands()]
    return DpmAgentQuery.BooleanExpression().set_op(dpm_boolean_op).set_arguments_list(args)

def make_dpm_order_by_expression(ordering: Ordering) -> DpmAgentQuery.OrderByExpression:
    field_expr, direction = ordering
    dpm_direction = DpmAgentQuery.OrderByExpression.Direction.ASC if direction == 'ASC' else DpmAgentQuery.OrderByExpression.Direction.DESC
    return DpmAgentQuery.OrderByExpression().set_argument(make_dpm_expression(field_expr)).set_direction(dpm_direction)

class DpmAgentClient:
    def __init__(self, service_address: str, creds: ChannelCredentials, connection_request: ConnectionRequest):
        print('Attempting to connect to', service_address)
        self.client = DpmAgentGrpcClient(service_address, creds)
        self.connection_id = self.client.create_connection(connection_request).connectionid

    async def make_dpm_agent_query(self, query: Table) -> DpmAgentQuery:
        dpm_agent_query = DpmAgentQuery()
        dpm_agent_query.set_connectionid(await self.connection_id)
        dpm_agent_query.set_selectfrom(query.name)

        filter_expr, selection, ordering, limit_to = query.filter_expr, query.selection, query.ordering, query.limit_to

        selections = list(map(make_dpm_select_expression, selection)) if selection else None
        if selections:
            dpm_agent_query.set_select_list(selections)

        if filter_expr:
            dpm_agent_query.set_filter(make_dpm_boolean_expression(filter_expr))

        if selection and any(isinstance(field_expr, AggregateFieldExpr) for field_expr in selection):
            grouping = list(filter(lambda field_expr: not isinstance(field_expr, AggregateFieldExpr), selection))
            if grouping:
                dpm_agent_query.set_groupby_list(list(map(make_dpm_group_by_expression, grouping)))

        if ordering and len(ordering) > 0:
            dpm_orderings = list(map(make_dpm_order_by_expression, ordering))
            dpm_agent_query.set_orderby_list(dpm_orderings)

        if limit_to > 0:
            dpm_agent_query.set_limit(limit_to)

        return dpm_agent_query

    async def compile(self, query: Table) -> str:
        dpm_agent_query = await self.make_dpm_agent_query(query)
        response = self.client.compile_query(dpm_agent_query)
        return response.result

    async def execute(self, query: Table) -> List[Tuple[str, int]]:
        dpm_agent_query = await self.make_dpm_agent_query(query)
        response = self.client.execute_query(dpm_agent_query)

        try:
            json_data = json.loads(response.jsondata)
        except Exception as e:
            print('dpm-agent: Error parsing results', e)
            raise ValueError('Error parsing JSON', e)

        return json_data
