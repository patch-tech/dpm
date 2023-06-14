import grpc

from backends.dpm_agent.dpm_agent_client import DpmAgentClient
from backends.dpm_agent.dpm_agent_pb2 import (
    ConnectionRequest,
    SnowflakeConnectionParams,
)


class Snowflake(DpmAgentClient):
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
        connection_request.snowflakeconnectionparams.CopyFrom(snowflake_connection_params)

        super().__init__(
            dpm_agent_service_address,
            grpc.ChannelCredentials.create_insecure(),
            connection_request,
        )
