import json
import logging
from typing import Dict, List, Union
from urllib.parse import urlparse

import grpc

from ...field import (
    AggregateFieldExpr,
    BooleanFieldExpr,
    DerivedField,
    FieldExpr,
    LiteralField,
    Scalar,
    UnaryBooleanFieldExpr,
)
from ...version import CODE_VERSION
from .dpm_agent_pb2 import ClientVersion
from .dpm_agent_pb2 import Query as DpmAgentQuery
from .dpm_agent_pb2_grpc import DpmAgentStub as DpmAgentGrpcClient

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
    "sum": DpmAgentQuery.AggregateExpression.SUM,
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
    "dayOfWeek": DpmAgentQuery.DerivedExpression.DAY_OF_WEEK,
    "week": DpmAgentQuery.DerivedExpression.WEEK,
    "weekDate": DpmAgentQuery.DerivedExpression.DATE_OF_WEEK,
    "date": DpmAgentQuery.DerivedExpression.DATE,
    "time": DpmAgentQuery.DerivedExpression.TIME,
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
    "hasAny": DpmAgentQuery.BooleanExpression.BooleanOperator.HAS_ANY,
    "hasAll": DpmAgentQuery.BooleanExpression.BooleanOperator.HAS_ALL,
}


def make_dpm_boolean_expression(
    filter: Union[BooleanFieldExpr, UnaryBooleanFieldExpr],
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


def make_dpm_agent_query(query) -> DpmAgentQuery:
    """
    Makes a query message from the table expression to send to dpm-agent.

    Args:
        query: Table expression

    Returns:
        Query RPC message to send to dpm-agent.
    """
    dpm_agent_query = DpmAgentQuery()
    dpm_agent_query.id.packageId = query.package_id
    dpm_agent_query.clientVersion.CopyFrom(
        ClientVersion(
            client=ClientVersion.PYTHON,
            codeVersion=CODE_VERSION,
            datasetVersion=query.dataset_version,
        )
    )
    dpm_agent_query.selectFrom = query.name

    filter_expr, selection, ordering, limit_to = (
        query.filter_expr,
        query.selection,
        query.ordering,
        query.limit_to,
    )

    dpm_select_exprs = (
        list(map(make_dpm_select_expression, selection)) if selection else None
    )
    if dpm_select_exprs:
        dpm_agent_query.select.extend(dpm_select_exprs)

    if filter_expr:
        dpm_agent_query.filter.CopyFrom(make_dpm_boolean_expression(filter_expr))

    selection = selection or []
    ordering = ordering or []
    # NB: we cannot use the selection expressions themselves as elements in a
    # set as they are not hashable, and their __eq__ magic method has been
    # overridden to support their usage in query filter expressions. We use the
    # set of field expression names to determine if an order-by expression is
    # already present in the selection set.
    selection_set = set([x.name for x in selection])
    expanded_selection = selection[:]
    expanded_selection.extend([x for x, _ in ordering if x.name not in selection_set])
    if expanded_selection and any(
        isinstance(x, AggregateFieldExpr) for x in expanded_selection
    ):
        grouping = [
            x for x in expanded_selection if not isinstance(x, AggregateFieldExpr)
        ]
        if grouping:
            dpm_agent_query.groupBy.extend(map(make_dpm_group_by_expression, grouping))

    if ordering and len(ordering) > 0:
        dpm_agent_query.orderBy.extend(map(make_dpm_order_by_expression, ordering))

    if limit_to > 0:
        dpm_agent_query.limit = limit_to

    return dpm_agent_query


class DpmAgentClient:
    """DpmAgentClient uses a gRPC client to compile and execute queries by using
    the `dpm-agent` which routes the queries to the specific source specified in
    the query's package descriptor."""

    def __init__(
        self,
        client: DpmAgentGrpcClient,
        dpm_auth_token: str,
    ):
        self.client = client
        self.dpm_auth_token = dpm_auth_token
        # NOTE: gRPC metadata keys are case insensitive according to:
        # https://grpc.io/docs/what-is-grpc/core-concepts/#metadata
        # However, specifying uppercase characters in the key throws a
        # ValueError: metadata was invalid
        self.metadata = [(b"dpm-auth-token", bytes(self.dpm_auth_token, "utf-8"))]

    async def compile(self, query) -> str:
        """
        Compiles table expression using dpm-agent.

        Args:
            query: Table expression to compile.

        Returns:
            Resolves to the compiled query string obtained from dpm-agent, or rejects on error.
        """
        dpm_agent_query = make_dpm_agent_query(query)
        dpm_agent_query.dryRun = True
        response = self.client.ExecuteQuery(dpm_agent_query, metadata=self.metadata)
        return response.queryString

    async def execute(self, query) -> List[Dict]:
        """
        Executes table expression using dpm-agent.

        Args:
            query: Table expression to execute.

        Returns:
            Resolves to the executed query results obtained from dpm-agent, or rejects on error.
        """
        dpm_agent_query = make_dpm_agent_query(query)
        response = self.client.ExecuteQuery(dpm_agent_query, metadata=self.metadata)

        try:
            json_data = json.loads(response.jsonData)
        except Exception as e:
            logger.error("dpm-agent: Error parsing results", e)
            raise ValueError("Error parsing JSON", e)

        return json_data


# A cache of gRPC clients keyed by service address so we create a single client
# per service address.
grpc_client_for_address = {}


def make_client(
    dpm_agent_address: str,
    dpm_auth_token: str,
) -> DpmAgentClient:
    """A factory for creating DpmAgentClient instances that share a single gRPC
    client to a given service address (must be valid URL with scheme).

    Args:
        dpm_agent_address: A valid URL string pointing to a `dpm-agent` server.
            (e.g. 'http://localhost:50051', 'https://agent.dpm.sh')
        dpm_auth_token: Token to authenticate with the `dpm-agent`. Obtained
            using `dpm login`.

    Returns:
        An instance of DpmAgentClient that can be used to call the specified
        `dpm-agent` instance.
    """

    if dpm_agent_address in grpc_client_for_address:
        grpc_client = grpc_client_for_address[dpm_agent_address]
    else:
        logger.info(f"Attempting to connect to {dpm_agent_address}")

        dpm_agent_url = urlparse(dpm_agent_address)
        # If the service address has an `https` scheme, or has port 443, create a
        # secure channel with TLS credentials.
        channel: grpc.Channel = None
        # NB: gRPC channel creation requires the network location of the service
        # address.  i.e., the {hostname} or {hostname}:{port} part of the URL.
        # Including the protocol prefix results in a DNS failure.
        if dpm_agent_url.scheme == "https" or dpm_agent_url.port == 443:
            channel = grpc.secure_channel(
                dpm_agent_url.netloc, grpc.ssl_channel_credentials()
            )
        else:
            channel = grpc.insecure_channel(dpm_agent_url.netloc)

        grpc_client = DpmAgentGrpcClient(channel)
        grpc_client_for_address[dpm_agent_address] = grpc_client

    return DpmAgentClient(grpc_client, dpm_auth_token)
