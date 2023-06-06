// Implements the Snowflake backend using DpmAgentClient.

import { credentials } from '@grpc/grpc-js';
import { DpmAgentClient } from './dpm_agent/dpm_agent_client';
import {
  ConnectionRequest,
  SnowflakeConnectionParams,
} from './dpm_agent/dpm_agent_pb';

export class Snowflake extends DpmAgentClient {
  constructor(
    dpmAgentServiceAddress: string,
    account: string,
    user: string,
    password: string,
    database: string,
    schema: string
  ) {
    const connectionRequest = new ConnectionRequest();
    const snowflakeConnectionParams = new SnowflakeConnectionParams()
      .setAccount(account)
      .setUser(user)
      .setPassword(password)
      .setDatabase(database)
      .setSchema(schema);
    connectionRequest.setSnowflakeconnectionparams(snowflakeConnectionParams);

    super(dpmAgentServiceAddress, credentials.createInsecure(), connectionRequest);
  }
}
