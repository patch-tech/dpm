// Factory to create the execution backend instance based on the query table's source.
import { Table } from '../table';
import { getEnv } from './env';
import { Backend } from './interface';
import { Patch } from './patch';
import { Snowflake } from './snowflake';

enum SourceType {
  UNKNOWN,
  PATCH_GRAPHQL,
  SNOWFLAKE,
}

function getSourceType(source: string): SourceType {
  let url: URL | undefined;
  try {
    url = new URL(source);
  } catch (e) { }

  if (url?.hostname === 'api.patch.tech' && url.pathname === '/query/graphql') {
    return SourceType.PATCH_GRAPHQL;
  } else if (url?.hostname.endsWith('snowflakecomputing.com')) {
    return SourceType.SNOWFLAKE;
  }

  return SourceType.UNKNOWN;
}

export function makeBackend(query: Table): Backend | undefined {
  const {
    dataset: { version },
    name,
    source,
  } = query;
  if (!source) {
    throw new Error(
      'Cannot get execution backend for query with unknown source'
    );
  }

  const sourceType = getSourceType(source);
  switch (sourceType) {
    case SourceType.PATCH_GRAPHQL:
      const authToken: string = getEnv('PATCH_AUTH_TOKEN');
      return new Patch(source, name, version, authToken);
    case SourceType.SNOWFLAKE:
      const dpmAgentHost = getEnv('DPM_AGENT_HOST', 'localhost');
      const dpmAgentPort = getEnv('DPM_AGENT_PORT', '50051');
      const snowflakeAccount = getEnv('SNOWSQL_ACCOUNT');
      const snowflakeUser = getEnv('SNOWSQL_USER');
      const snowflakePassword = getEnv('SNOWSQL_PWD');
      const snowflakeDatabase = getEnv('SNOWSQL_DATABASE');
      const snowflakeSchema = getEnv('SNOWSQL_SCHEMA');
      return new Snowflake(
        `${dpmAgentHost}:${dpmAgentPort}`,
        snowflakeAccount,
        snowflakeUser,
        snowflakePassword,
        snowflakeDatabase,
        snowflakeSchema
      );
    default:
      console.log(
        `Unknown source type, ${sourceType}, for query's table source ${source}`
      );
  }
}