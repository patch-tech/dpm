from datetime import date
import pytest

from ..table import Table
from ..field import DateField, Field, StringField
from ..version import CODE_VERSION
from ..backends.dpm_agent.dpm_agent_client import make_dpm_agent_query
from ..backends.dpm_agent.dpm_agent_pb2 import ClientVersion, Query as DpmAgentQuery


@pytest.fixture
def id():
    return StringField("id")


@pytest.fixture
def name():
    return StringField("name")


@pytest.fixture
def price():
    return Field("price")


@pytest.fixture
def created_on():
    return DateField("created_on")


@pytest.fixture
def table(id, name, price, created_on):
    backend = {}
    return Table(
        backend=backend,
        package_id="pkg-123",
        dataset_name="ds=456",
        dataset_version="0.1.0",
        source="test",
        name="testTable",
        fields=[id, name, price, created_on],
    )


def test_returns_expected_query_message_for_query_with_selections(table):
    query = table.select("id", "name").limit(10)
    dpm_agent_query = make_dpm_agent_query(query)
    assert dpm_agent_query

    want = DpmAgentQuery(
        id={"packageId": "pkg-123"},
        clientVersion={
            "client": ClientVersion.Client.PYTHON,
            "datasetVersion": "0.1.0",
            "codeVersion": "0.1.0",
        },
        selectFrom="testTable",
        select=[
            {"argument": {"field": {"fieldName": "id"}}},
            {"argument": {"field": {"fieldName": "name"}}},
        ],
        limit=10,
    )
    assert dpm_agent_query == want


def test_returns_expected_query_message_for_query_with_selections_and_filter(
    table, name, created_on
):
    query = (
        table.select("id", "name")
        .filter(name.like("%bah%") & (created_on < date(2023, 1, 1)))
        .limit(10)
    )
    dpm_agent_query = make_dpm_agent_query(query)
    assert dpm_agent_query

    want = DpmAgentQuery(
        id={"packageId": "pkg-123"},
        clientVersion={
            "client": ClientVersion.Client.PYTHON,
            "datasetVersion": "0.1.0",
            "codeVersion": "0.1.0",
        },
        selectFrom="testTable",
        select=[
            {"argument": {"field": {"fieldName": "id"}}},
            {"argument": {"field": {"fieldName": "name"}}},
        ],
        filter={
            "op": DpmAgentQuery.BooleanExpression.AND,
            "arguments": [
                {
                    "condition": {
                        "op": DpmAgentQuery.BooleanExpression.LIKE,
                        "arguments": [
                            {"field": {"fieldName": "name"}},
                            {"literal": {"string": "%bah%"}},
                        ],
                    }
                },
                {
                    "condition": {
                        "op": DpmAgentQuery.BooleanExpression.LT,
                        "arguments": [
                            {"field": {"fieldName": "created_on"}},
                            {"literal": {"string": "2023-01-01"}},
                        ],
                    }
                },
            ],
        },
        limit=10,
    )
    assert dpm_agent_query == want


def test_returns_expected_query_message_for_query_with_selections_filter_aggs_order(
    table, name, created_on, price
):
    query = (
        table.select("id", "name", price.avg().with_alias("avg_price"))
        .filter(name.like("%bah%") & (created_on < date(2023, 1, 1)))
        .order_by(["avg_price", "DESC"], [created_on, "ASC"])
        .limit(10)
    )
    dpm_agent_query = make_dpm_agent_query(query)
    assert dpm_agent_query

    want = DpmAgentQuery(
        id={"packageId": "pkg-123"},
        clientVersion={
            "client": ClientVersion.Client.PYTHON,
            "datasetVersion": "0.1.0",
            "codeVersion": "0.1.0",
        },
        selectFrom="testTable",
        select=[
            {"argument": {"field": {"fieldName": "id"}}},
            {"argument": {"field": {"fieldName": "name"}}},
            {
                "argument": {
                    "aggregate": {
                        "op": DpmAgentQuery.AggregateExpression.MEAN,
                        "argument": {"field": {"fieldName": "price"}},
                    }
                },
                "alias": "avg_price",
            },
        ],
        filter={
            "op": DpmAgentQuery.BooleanExpression.AND,
            "arguments": [
                {
                    "condition": {
                        "op": DpmAgentQuery.BooleanExpression.LIKE,
                        "arguments": [
                            {"field": {"fieldName": "name"}},
                            {"literal": {"string": "%bah%"}},
                        ],
                    }
                },
                {
                    "condition": {
                        "op": DpmAgentQuery.BooleanExpression.LT,
                        "arguments": [
                            {"field": {"fieldName": "created_on"}},
                            {"literal": {"string": "2023-01-01"}},
                        ],
                    }
                },
            ],
        },
        groupBy=[
            {"field": {"fieldName": "id"}},
            {"field": {"fieldName": "name"}},
            {"field": {"fieldName": "created_on"}},
        ],
        orderBy=[
            {
                "argument": {
                    "aggregate": {
                        "op": DpmAgentQuery.AggregateExpression.MEAN,
                        "argument": {"field": {"fieldName": "price"}},
                    },
                },
                "direction": DpmAgentQuery.OrderByExpression.DESC,
            },
            {
                "argument": {"field": {"fieldName": "created_on"}},
                "direction": DpmAgentQuery.OrderByExpression.ASC,
            },
        ],
        limit=10,
    )
    assert dpm_agent_query == want
