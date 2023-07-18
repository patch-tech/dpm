/**
 * Implements the Snowflake backend using DpmAgentClient.
 */
import { DpmAgentClient, makeClient } from './dpm_agent/dpm_agent_client';
import {
  ConnectionRequest,
  SnowflakeConnectionParams,
  ClientVersion,
} from './dpm_agent/dpm_agent_pb';

import { Table } from '../table';
import { Backend } from './interface';
import { codeVersion } from '../version';

export class Snowflake implements Backend {
  private dpmAgentClient: DpmAgentClient;

  /**
   * Constructs a Snowflake backend via dpm-agent.
   * @param dpmAgentServiceAddress The dpm-agent address in {host}:{port} format.
   * @param account
   * @param user
   * @param password
   * @param database
   * @param schema
   */
  constructor(
    dpmAgentServiceAddress: string,
    account: string,
    user: string,
    password: string,
    database: string,
    schema: string,
    datasetVersion: string
  ) {
    const connectionRequest = new ConnectionRequest();
    const snowflakeConnectionParams = new SnowflakeConnectionParams()
      .setAccount(account)
      .setUser(user)
      .setPassword(password)
      .setDatabase(database)
      .setSchema(schema);
    connectionRequest.setSnowflakeconnectionparams(snowflakeConnectionParams);
    const clientVersion = new ClientVersion()
      .setClient(ClientVersion.Client.NODE_JS)
      .setDatasetversion(datasetVersion)
      .setCodeversion(codeVersion);
    connectionRequest.setClientversion(clientVersion);

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
