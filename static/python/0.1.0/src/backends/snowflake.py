import grpc

from backends.dpm_agent.dpmAgentClient import DpmAgentClient
from backends.dpm_agent.dpm_agent_pb2 import (
    ConnectionRequest,
    SnowflakeConnectionParams,
)


class Snowflake(DpmAgentClient):
    def __init__(
        self,
        dpmAgentServiceAddress: str,
        account: str,
        user: str,
        password: str,
        database: str,
        schema: str,
    ):
        connectionRequest = ConnectionRequest()
        snowflakeConnectionParams = SnowflakeConnectionParams(
            account=account,
            user=user,
            password=password,
            database=database,
            schema=schema,
        )
        connectionRequest.snowflakeconnectionparams.CopyFrom(snowflakeConnectionParams)

        super().__init__(
            dpmAgentServiceAddress,
            grpc.ChannelCredentials.create_insecure(),
            connectionRequest,
        )
