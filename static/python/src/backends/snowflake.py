"""Defines the Snowflake execution backend. Uses `dpm-agent` to compile and
execute queries."""

from typing import Dict, List, Literal, Union

from ..backends.dpm_agent.dpm_agent_client import make_client
from ..backends.dpm_agent.dpm_agent_pb2 import (
    ConnectionRequest,
    SnowflakeConnectionParams,
)
from .interface import Backend


class Snowflake(Backend):
    def __init__(
        self,
        dpm_agent_service_address: str,
        account: str,
        user: str,
        password: str,
        database: str,
        schema: str,
    ):
        connection_request = ConnectionRequest()
        snowflake_connection_params = SnowflakeConnectionParams(
            account=account,
            user=user,
            password=password,
            database=database,
            schema=schema,
        )
        connection_request.snowflakeConnectionParams.CopyFrom(
            snowflake_connection_params
        )

        self.dpm_agent_client = make_client(
            dpm_agent_service_address, connection_request
        )

    async def compile(self, query) -> str:
        return self.dpm_agent_client.compile(query)

    async def execute(self, query) -> List[Dict]:
        return self.execute(query)
