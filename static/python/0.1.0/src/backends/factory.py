from typing import Optional
from urllib.parse import urlparse
from table import Table
from env import getEnv
from interface import Backend
from patch import Patch
from snowflake import Snowflake

from enum import Enum

class SourceType(Enum):
    UNKNOWN = 0
    PATCH_GRAPHQL = 1
    SNOWFLAKE = 2

def getSourceType(source: str) -> SourceType:
    url = urlparse(source)

    if url.hostname == 'api.patch.tech' and url.path == '/query/graphql':
        return SourceType.PATCH_GRAPHQL
    elif url.hostname.endswith('snowflakecomputing.com'):
        return SourceType.SNOWFLAKE

    return SourceType.UNKNOWN

def makeBackend(query: Table) -> Optional[Backend]:
    version = query.dataset.version
    name = query.name
    source = query.source

    if not source:
        raise ValueError('Cannot get execution backend for query with unknown source')

    sourceType = getSourceType(source)

    if sourceType == SourceType.PATCH_GRAPHQL:
        authToken = getEnv('PATCH_AUTH_TOKEN')
        return Patch(source, name, version, authToken)
    elif sourceType == SourceType.SNOWFLAKE:
        dpmAgentHost = getEnv('DPM_AGENT_HOST', 'localhost')
        dpmAgentPort = getEnv('DPM_AGENT_PORT', '50051')
        snowflakeAccount = getEnv('SNOWFLAKE_ACCOUNT')
        snowflakeUser = getEnv('SNOWFLAKE_USER')
        snowflakePassword = getEnv('SNOWFLAKE_PASSWORD')
        snowflakeDatabase = getEnv('SNOWFLAKE_DATABASE')
        snowflakeSchema = getEnv('SNOWFLAKE_SCHEMA')
        return Snowflake(
            f"{dpmAgentHost}:{dpmAgentPort}",
            snowflakeAccount,
            snowflakeUser,
            snowflakePassword,
            snowflakeDatabase,
            snowflakeSchema
        )
    else:
        print(f"Unknown source type, {sourceType}, for query's table source {source}")
