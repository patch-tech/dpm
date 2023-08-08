/**
 * Factory to create the execution backend instance based on the query table's source.
 */
import { Table } from '../table';
import { getEnv } from './env';
import { Backend } from './interface';
import { Patch } from './patch';
import { Snowflake } from './snowflake';
import * as fs from 'fs';

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
 * Discovers the `dpm` authentication token by inspecting:
 * 1. Environment variable DPM_AUTH_TOKEN
 * 2. The session.json file stored by `dpm login`.
 * 3. ...
 */
function getDpmAuthToken(): string | undefined {
  let dpmAuthToken = getEnv('DPM_AUTH_TOKEN', undefined);
  if (dpmAuthToken !== undefined) {
    return dpmAuthToken;
  }
  interface Session {
    access_token: string;
    token_type: number;
    expires_in: string;
    scope: string;
  }
  let root_dir = process.env.HOME
  let session_dir = ''

  if (process.platform == 'darwin') {
    session_dir = root_dir + '/Library/Application Support/tech.patch.dpm/session.json'
  } else if (process.platform == 'win32') {
    session_dir = root_dir + '\\AppData\\Roaming\\patch\\session.json'
  } else if (process.platform == 'linux') {
    session_dir = root_dir + '.config/dpm/session.json'
  }
  try {
    const sessionString = fs.readFileSync(session_dir, 'utf-8');
    const sessionData: Session = JSON.parse(sessionString);
    return sessionData.access_token
  } catch (err) {
    console.error("error recieving access token from project directory:", err);
  }
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
  const dpmAuthToken = getDpmAuthToken();
  if (dpmAuthToken === undefined) {
    throw new Error(
      'Failed to find DPM authentication token. Please run `dpm login`'
    );
  }

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
        dpmAuthToken,
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
