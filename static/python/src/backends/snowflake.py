"""Defines the Snowflake execution backend using DpmAgentClient."""

from typing import Dict, List

from ..backends.dpm_agent.dpm_agent_client import DpmAgentClient, make_client
from ..backends.dpm_agent.dpm_agent_pb2 import (
    ClientVersion,
    ConnectionRequest,
    SnowflakeConnectionParams,
)
from ..version import CODE_VERSION
from .interface import Backend


class Snowflake(Backend):
    def __init__(
        self,
        dpm_agent_service_address: str,
        dpm_auth_token: str,
        account: str,
        user: str,
        password: str,
        database: str,
        schema: str,
        dataset_version: str,
    ):
        """
        Constructs a Snowflake backend via dpm-agent.

        Args:
            dpm_agent_service_address: The dpm-agent address in {host}:{port} format.
            dpm_auth_token: Token to authenticate with the `dpm-agent`. Obtained using `dpm login`.
            account: Snowflake account name.
            user: Snowflake user name.
            password: Snowflake user password.
            database: Snowflake database name.
            schema: Snowflake schema name.
        """
        self._dpm_agent_service_address = dpm_agent_service_address
        self._dpm_auth_token = dpm_auth_token
        self._connection_request = ConnectionRequest()
        snowflake_connection_params = SnowflakeConnectionParams(
            account=account,
            user=user,
            password=password,
            database=database,
            schema=schema,
        )
        self._connection_request.snowflakeConnectionParams.CopyFrom(
            snowflake_connection_params
        )
        self._connection_request.clientVersion.CopyFrom(
            ClientVersion(
                client=ClientVersion.PYTHON,
                codeVersion=CODE_VERSION,
                datasetVersion=dataset_version,
            )
        )
        self.dpm_agent_client = None

    async def get_or_make_dpm_agent_client(self) -> DpmAgentClient:
        if self.dpm_agent_client is None:
            self.dpm_agent_client = await make_client(
                self._dpm_agent_service_address,
                self._dpm_auth_token,
                self._connection_request,
            )
        return self.dpm_agent_client

    async def compile(self, query) -> str:
        return await (await self.get_or_make_dpm_agent_client()).compile(query)

    async def execute(self, query) -> List[Dict]:
        return await (await self.get_or_make_dpm_agent_client()).execute(query)
