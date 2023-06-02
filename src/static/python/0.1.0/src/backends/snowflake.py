import grpc
from dpm_agent.dpmAgentClient import DpmAgent_pb, DpmAgent_pb_grpc

class Snowflake(DpmAgent_pb_grpc.DpmAgentClient):
    def __init__(self, dpmAgentServiceAddress: str, account: str, user: str, password: str, database: str, schema: str):
        connectionRequest = DpmAgent_pb.ConnectionRequest()
        snowflakeConnectionParams = DpmAgent_pb.SnowflakeConnectionParams(
            account=account,
            user=user,
            password=password,
            database=database,
            schema=schema
        )
        connectionRequest.snowflakeconnectionparams.CopyFrom(snowflakeConnectionParams)

        super().__init__(dpmAgentServiceAddress, grpc.ChannelCredentials.create_insecure(), connectionRequest)
