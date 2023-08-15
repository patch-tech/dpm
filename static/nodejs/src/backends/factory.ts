/**
 * Factory to create an execution backend instance.
 */
import { getEnv } from './env';
import { Backend } from './interface';
import * as fs from 'fs';
import os from 'os';
import path from 'node:path';
import { makeClient } from './dpm_agent/dpm_agent_client';

/**
 * Discovers the `dpm` authentication token by inspecting:
 * 1. Environment variable DPM_AUTH_TOKEN
  * 2. The session.json file stored by `dpm login`.
  * 3. ...
  */
export function getDpmAuthToken(): string | undefined {
  try {
    let dpmAuthToken = getEnv('DPM_AUTH_TOKEN', undefined);
    if (dpmAuthToken !== undefined) {
      return dpmAuthToken;
    }
  } catch {
    interface Session {
      access_token: string;
      token_type: number;
      expires_in: string;
      scope: string;
    }
    let rootDir = os.homedir()
    let sessionPath = ''

    if (process.platform == 'darwin') {
      sessionPath = path.join(rootDir, 'Library', 'Application Support', 'tech.patch.dpm', 'session.json')
    } else if (process.platform == 'win32') {
      sessionPath = path.join(rootDir, 'AppData', 'Roaming', 'patch', 'session.json')
    } else if (process.platform == 'linux') {
      sessionPath = path.join(rootDir, '.config', 'dpm', 'session.json')
    }
    try {
      const sessionString = fs.readFileSync(sessionPath, 'utf-8');
      const sessionData: Session = JSON.parse(sessionString);
      return sessionData.access_token
    } catch (err) {
      console.error("error recieving access token from project directory:", err);
    }
  }
}

/**
 * Makes an instance of the backend that can communicate with `dpm-agent` to
 * compile and execute queries.
 * @returns A Backend instance.
 */
export function makeBackend(): Backend {
  const dpmAuthToken = getDpmAuthToken();
  if (dpmAuthToken === undefined) {
    throw new Error(
      'Failed to find DPM authentication token. Please run `dpm login`'
    );
  }

  const dpmAgentServiceAddress = getEnv('DPM_AGENT_URL', 'https://agent.dpm.sh');
  return makeClient({dpmAgentServiceAddress, dpmAuthToken});
}
