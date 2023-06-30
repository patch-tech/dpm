"""Defines the Snowflake execution backend. Uses `dpm-agent` to compile and
execute queries."""

from typing import Dict, List
import logging
import signal
import asyncio

from ..backends.dpm_agent.dpm_agent_client import DpmAgentClient, make_client, close_all_clients_and_connections
from ..backends.dpm_agent.dpm_agent_pb2 import (
    ConnectionRequest,
    SnowflakeConnectionParams,
)
from .interface import Backend


async def close_dpm_client():
    logging.warn('Closing dpm agent client')
    await close_all_clients_and_connections()

def close_on_interrupt():
    loop = asyncio.get_event_loop()
    for signame in ('SIGINT', 'SIGTERM'):
        loop.add_signal_handler(getattr(signal, signame),
                                lambda: asyncio.create_task(close_dpm_client()))

import atexit
@atexit.register
def shutdown():
    asyncio.run(close_dpm_client())

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
        self._dpm_agent_service_address = dpm_agent_service_address
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
        self.dpm_agent_client = None

    async def get_or_make_dpm_agent_client(self) -> DpmAgentClient:
        if self.dpm_agent_client is None:
            close_on_interrupt()
            self.dpm_agent_client = await make_client(
                self._dpm_agent_service_address, self._connection_request
            )
        return self.dpm_agent_client

    async def compile(self, query) -> str:
        return await (await self.get_or_make_dpm_agent_client()).compile(query)

    async def execute(self, query) -> List[Dict]:
        return await (await self.get_or_make_dpm_agent_client()).execute(query)
