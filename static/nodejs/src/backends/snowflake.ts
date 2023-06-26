// Implements the Snowflake backend using DpmAgentClient.

import { DpmAgentClient, makeClient } from './dpm_agent/dpm_agent_client';
import {
  ConnectionRequest,
  SnowflakeConnectionParams,
} from './dpm_agent/dpm_agent_pb';

import { Table } from '../table';
import { Backend } from './interface';

export class Snowflake implements Backend {
  private dpmAgentClient: DpmAgentClient;
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

    this.dpmAgentClient = makeClient({
      dpmAgentServiceAddress,
      connectionRequest,
    });
  }

  async compile(query: Table): Promise<string> {
    return this.dpmAgentClient.compile(query);
  }

  async execute<Row>(query: Table): Promise<Row[]> {
    return this.dpmAgentClient.execute(query);
  }
}
