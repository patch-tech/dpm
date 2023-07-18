/**
 * Factory to create the execution backend instance based on the query table's source.
 */
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

/**
 * Returns the source type of the input source url
 * @param source A valid URL string pointing to the source of the data resource.
 * @returns A source type, if supported; unknown if not supported.
 */
function getSourceType(source: string): SourceType {
  let url: URL | undefined;
  try {
    url = new URL(source);
  } catch (e) {}

  if (url?.hostname === 'api.patch.tech' && url.pathname === '/query/graphql') {
    return SourceType.PATCH_GRAPHQL;
  } else if (url?.hostname.endsWith('snowflakecomputing.com')) {
    return SourceType.SNOWFLAKE;
  }

  return SourceType.UNKNOWN;
}

/**
 * Makes an instance of the backend that can communicate with the source that
 * holds the table's data.
 * @param table Table expression that can be executed against the created backend.
 * @returns A Backend instance or undefined if the source is not supported.
 */
export function makeBackend(table: Table): Backend | undefined {
  const { datasetVersion, name, source } = table;
  if (!source) {
    throw new Error(
      'Cannot get execution backend for query with unknown source'
    );
  }

  const sourceType = getSourceType(source);
  switch (sourceType) {
    case SourceType.PATCH_GRAPHQL:
      const authToken: string = getEnv('PATCH_AUTH_TOKEN');
      return new Patch(source, name, datasetVersion, authToken);
    case SourceType.SNOWFLAKE:
      const dpmAgentUrl = getEnv('DPM_AGENT_URL', 'https://agent.dpm.sh');
      const snowflakeAccount = getEnv('SNOWSQL_ACCOUNT');
      const snowflakeUser = getEnv('SNOWSQL_USER');
      const snowflakePassword = getEnv('SNOWSQL_PWD');
      const snowflakeDatabase = getEnv('SNOWSQL_DATABASE');
      const snowflakeSchema = getEnv('SNOWSQL_SCHEMA');
      return new Snowflake(
        dpmAgentUrl,
        snowflakeAccount,
        snowflakeUser,
        snowflakePassword,
        snowflakeDatabase,
        snowflakeSchema,
        datasetVersion
      );
    default:
      console.log(
        `Unknown source type, ${sourceType}, for query's table source ${source}`
      );
  }
}
