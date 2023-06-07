from typing import Optional
from urllib.parse import urlparse
from .env import get_env
from .interface import Backend
from .patch import Patch
from .snowflake import Snowflake

from enum import Enum


class SourceType(Enum):
    UNKNOWN = 0
    PATCH_GRAPHQL = 1
    SNOWFLAKE = 2


def get_source_type(source: str) -> SourceType:
    url = urlparse(source)

    if url.hostname == "api.patch.tech" and url.path == "/query/graphql":
        return SourceType.PATCH_GRAPHQL
    elif url.hostname.endswith("snowflakecomputing.com"):
        return SourceType.SNOWFLAKE

    return SourceType.UNKNOWN


def make_backend(query) -> Optional[Backend]:
    version = query.dataset.version
    name = query.name
    source = query.source

    if not source:
        raise ValueError("Cannot get execution backend for query with unknown source")

    sourceType = get_source_type(source)

    if sourceType == SourceType.PATCH_GRAPHQL:
        authToken = get_env("PATCH_AUTH_TOKEN")
        return Patch(source, name, version, authToken)
    elif sourceType == SourceType.SNOWFLAKE:
        dpmAgentHost = get_env('DPM_AGENT_HOST', 'localhost')
        dpmAgentPort = get_env('DPM_AGENT_PORT', '50051')
        snowflakeAccount = get_env('SNOWSQL_ACCOUNT')
        snowflakeUser = get_env('SNOWSQL_USER')
        snowflakePassword = get_env('SNOWSQL_PASSWORD')
        snowflakeDatabase = get_env('SNOWSQL_DATABASE')
        snowflakeSchema = get_env('SNOWSQL_SCHEMA')
        return Snowflake(
            f"{dpmAgentHost}:{dpmAgentPort}",
            snowflakeAccount,
            snowflakeUser,
            snowflakePassword,
            snowflakeDatabase,
            snowflakeSchema,
        )
    else:
        print(f'Unknown source type, "{sourceType}", for query\'s table source "{source}"')
