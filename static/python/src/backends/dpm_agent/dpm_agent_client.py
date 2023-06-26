import base64
from typing import List, Dict
import json
from grpc import Channel, RpcError
import logging
from .dpm_agent_pb2 import ConnectionRequest, ConnectionResponse, Query as DpmAgentQuery
from .dpm_agent_pb2_grpc import DpmAgentStub as DpmAgentGrpcClient
from ...field import (
    AggregateFieldExpr,
    BooleanFieldExpr,
    DerivedField,
    FieldExpr,
    LiteralField,
    Scalar,
)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def make_dpm_literal(literal: LiteralField) -> DpmAgentQuery.Literal:
    def make_literal(x: Scalar) -> DpmAgentQuery.Literal:
        if isinstance(x, str):
            return DpmAgentQuery.Literal(string=x)
        elif isinstance(x, int):
            return DpmAgentQuery.Literal(i64=x)
        elif isinstance(x, float):
            return DpmAgentQuery.Literal(f64=x)
        elif isinstance(x, bool):
            return DpmAgentQuery.Literal(boolean=x)

        # Must be a date type
        return DpmAgentQuery.Literal(timestamp=int(x))

    if isinstance(literal.value, list):
        return DpmAgentQuery.Literal(
            list=DpmAgentQuery.Literal.List(
                values=[make_literal(val) for val in literal.value]
            )
        )
    return make_literal(literal.value)


def make_dpm_field_reference(field: FieldExpr) -> DpmAgentQuery.FieldReference:
    return DpmAgentQuery.FieldReference(fieldName=str(field.operands()[0]))


AGGREGATE_OPERATOR_MAP = {
    "min": DpmAgentQuery.AggregateExpression.MIN,
    "max": DpmAgentQuery.AggregateExpression.MAX,
    "count": DpmAgentQuery.AggregateExpression.COUNT,
    "countDistinct": DpmAgentQuery.AggregateExpression.COUNT_DISTINCT,
    "avg": DpmAgentQuery.AggregateExpression.MEAN,
    "avgDistinct": DpmAgentQuery.AggregateExpression.MEAN,
}


def make_dpm_aggregate_expression(
    agg_expr: AggregateFieldExpr,
) -> DpmAgentQuery.AggregateExpression:
    base_field = agg_expr.operands()[0]
    base_dpm_expr = make_dpm_expression(base_field)
    agg_op = agg_expr.operator()
    dpm_agg_op = AGGREGATE_OPERATOR_MAP.get(agg_op)
    if dpm_agg_op is None:
        raise ValueError(f'Unsupported aggregate operation "{agg_op}"')

    return DpmAgentQuery.AggregateExpression(argument=base_dpm_expr, op=dpm_agg_op)


PROJECTION_OPERATOR_MAP = {
    "day": DpmAgentQuery.DerivedExpression.DAY,
    "month": DpmAgentQuery.DerivedExpression.MONTH,
    "year": DpmAgentQuery.DerivedExpression.YEAR,
    "hour": DpmAgentQuery.DerivedExpression.HOUR,
    "minute": DpmAgentQuery.DerivedExpression.MINUTE,
    "second": DpmAgentQuery.DerivedExpression.SECOND,
    "millisecond": DpmAgentQuery.DerivedExpression.MILLISECOND,
}


def make_dpm_derived_expression(
    derived_field: DerivedField,
) -> DpmAgentQuery.DerivedExpression:
    base_field = derived_field.operands()[0]
    base_dpm_expr = make_dpm_expression(base_field)
    projection_op = derived_field.operator()
    dpm_projection_op = PROJECTION_OPERATOR_MAP.get(projection_op)
    if dpm_projection_op is None:
        raise ValueError(f'Unsupported projection operation "{projection_op}"')

    return DpmAgentQuery.DerivedExpression(argument=base_dpm_expr, op=dpm_projection_op)


def make_dpm_expression(field: FieldExpr) -> DpmAgentQuery.Expression:
    if isinstance(field, LiteralField):
        return DpmAgentQuery.Expression(literal=make_dpm_literal(field))
    elif isinstance(field, AggregateFieldExpr):
        return DpmAgentQuery.Expression(aggregate=make_dpm_aggregate_expression(field))
    elif isinstance(field, DerivedField):
        return DpmAgentQuery.Expression(derived=make_dpm_derived_expression(field))
    elif field.operator() != "ident":
        raise ValueError(f'Unexpected field expression "{field}"')
    return DpmAgentQuery.Expression(field=make_dpm_field_reference(field))


def make_dpm_group_by_expression(field: FieldExpr) -> DpmAgentQuery.GroupByExpression:
    if isinstance(field, DerivedField):
        return DpmAgentQuery.GroupByExpression(
            derived=make_dpm_derived_expression(field)
        )
    elif field.operator() != "ident":
        raise ValueError(f'Unexpected field expression in groupBy: "{field}"')
    return DpmAgentQuery.GroupByExpression(field=make_dpm_field_reference(field))


def make_dpm_select_expression(field: FieldExpr) -> DpmAgentQuery.SelectExpression:
    select_expr = DpmAgentQuery.SelectExpression(argument=make_dpm_expression(field))

    if field.alias is not None:
        select_expr.alias = field.alias

    return select_expr


BOOLEAN_OPERATOR_MAP = {
    "and": DpmAgentQuery.BooleanExpression.BooleanOperator.AND,
    "or": DpmAgentQuery.BooleanExpression.BooleanOperator.OR,
    "eq": DpmAgentQuery.BooleanExpression.BooleanOperator.EQ,
    "neq": DpmAgentQuery.BooleanExpression.BooleanOperator.NEQ,
    "gt": DpmAgentQuery.BooleanExpression.BooleanOperator.GT,
    "gte": DpmAgentQuery.BooleanExpression.BooleanOperator.GTE,
    "lt": DpmAgentQuery.BooleanExpression.BooleanOperator.LT,
    "lte": DpmAgentQuery.BooleanExpression.BooleanOperator.LTE,
    "like": DpmAgentQuery.BooleanExpression.BooleanOperator.LIKE,
    "between": DpmAgentQuery.BooleanExpression.BooleanOperator.BETWEEN,
    "in": DpmAgentQuery.BooleanExpression.BooleanOperator.IN,
    "isNull": DpmAgentQuery.BooleanExpression.BooleanOperator.IS_NULL,
    "isNotNull": DpmAgentQuery.BooleanExpression.BooleanOperator.IS_NOT_NULL,
    # TODO(PAT-3175, PAT-3176): Define once we support unary not.
    "not": None,
    # TODO(PAT-3355): Remove `inPast` once we redefine it in terms of a `between` check.
    "inPast": None,
}


def make_dpm_boolean_expression(
    filter: BooleanFieldExpr,
) -> DpmAgentQuery.BooleanExpression:
    op = filter.operator()
    if op == "and" or op == "or":
        args = [
            DpmAgentQuery.Expression(condition=make_dpm_boolean_expression(expr))
            for expr in filter.operands()
        ]
        return DpmAgentQuery.BooleanExpression(
            op=BOOLEAN_OPERATOR_MAP[op], arguments=args
        )

    dpm_boolean_op = BOOLEAN_OPERATOR_MAP[op]
    if dpm_boolean_op is None:
        raise ValueError(f'Unhandled boolean operator "{op}"')

    args = [make_dpm_expression(expr) for expr in filter.operands()]
    return DpmAgentQuery.BooleanExpression(op=dpm_boolean_op, arguments=args)


def make_dpm_order_by_expression(ordering) -> DpmAgentQuery.OrderByExpression:
    field_expr, direction = ordering
    dpm_direction = (
        DpmAgentQuery.OrderByExpression.Direction.ASC
        if direction == "ASC"
        else DpmAgentQuery.OrderByExpression.Direction.DESC
    )
    return DpmAgentQuery.OrderByExpression(
        argument=make_dpm_expression(field_expr), direction=dpm_direction
    )


class DpmAgentClient:
    def __init__(
        self,
        client: DpmAgentGrpcClient,
        connection_id: str,
    ):
        self.client = client
        self.connection_id = connection_id

    async def make_dpm_agent_query(self, query) -> DpmAgentQuery:
        dpm_agent_query = DpmAgentQuery()
        dpm_agent_query.connectionId = self.connection_id
        dpm_agent_query.selectFrom = query.name

        filter_expr, selection, ordering, limit_to = (
            query.filter_expr,
            query.selection,
            query.ordering,
            query.limit_to,
        )

        selections = (
            list(map(make_dpm_select_expression, selection)) if selection else None
        )
        if selections:
            dpm_agent_query.select.extend(selections)

        if filter_expr:
            dpm_agent_query.filter.CopyFrom(make_dpm_boolean_expression(filter_expr))

        if selection and any(
            isinstance(field_expr, AggregateFieldExpr) for field_expr in selection
        ):
            grouping = filter(
                lambda field_expr: not isinstance(field_expr, AggregateFieldExpr),
                selection,
            )
            if grouping:
                dpm_agent_query.groupBy.extend(
                    map(make_dpm_group_by_expression, grouping)
                )

        if ordering and len(ordering) > 0:
            dpm_agent_query.orderBy.extend(map(make_dpm_order_by_expression, ordering))

        if limit_to > 0:
            dpm_agent_query.limit = limit_to

        return dpm_agent_query

    async def compile(self, query) -> str:
        dpm_agent_query = await self.make_dpm_agent_query(query)
        dpm_agent_query.dryRun = True
        response = self.client.ExecuteQuery(dpm_agent_query)
        return response.queryString

    async def execute(self, query) -> List[Dict]:
        dpm_agent_query = await self.make_dpm_agent_query(query)
        response = self.client.ExecuteQuery(dpm_agent_query)

        try:
            json_data = json.loads(response.jsonData)
        except Exception as e:
            logger.error("dpm-agent: Error parsing results", e)
            raise ValueError("Error parsing JSON", e)

        return json_data


# A dpm-agent gRPC client container that caches its execution backend
# connection ids, so we only create a single connection for a given execution
# backend, identity, and creds.
class DpmAgentGrpcClientContainer:
    def __init__(self, client: DpmAgentGrpcClient):
        self.client = client
        self.connection_id_for_req_ = {}

    async def connect(self, connection_request: ConnectionRequest) -> str:
        """Creates a connection to an execution backend, if one does not exist, and
        caches the connection id.  Returns the connection id obtained from
        `dpm-agent`."""
        req_str = base64.b64encode(connection_request.SerializeToString())

        if req_str not in self.connection_id_for_req_:
            try:
                response: ConnectionResponse = self.client.CreateConnection(
                    connection_request
                )
            except RpcError as error:
                logger.error("dpm-agent client: Error connecting...", error)
                raise Exception("Error connecting", {"cause": error})
            logger.debug(
                f"dpm-agent client: Connected, connection id: {response.connectionId}"
            )
            self.connection_id_for_req_[req_str] = response.connectionId
        return self.connection_id_for_req_[req_str]


# A cache of gRPC client containers keyed by service address so we create a
# single client per service address.
grpc_client_for_address = {}

def make_client(
    dpm_agent_service_address: str,
    connection_request: ConnectionRequest,
    creds=None,
) -> DpmAgentClient:
    """A factory for creating DpmAgentClient instances that share a single gRPC
    client to a given service address, and a single execution backend connection
    for a given set of identities and credentials."""
    if not creds:
        creds=grpc.insecure_channel(dpm_agent_service_address)

    if dpm_agent_service_address in grpc_client_for_address:
        client_container = grpc_client_for_address[dpm_agent_service_address]
    else:
        logger.info("Attempting to connect to", dpm_agent_service_address)
        grpc_client = DpmAgentGrpcClient(dpm_agent_service_address, creds)
        client_container = DpmAgentGrpcClientContainer(grpc_client)
        grpc_client_for_address[dpm_agent_service_address] = client_container

    connection_id = client_container.connect(connection_request)
    return DpmAgentClient(client_container.client, connection_id)
