"""Factory to create the execution backend instance based on the query table's source."""
from typing import Optional
from urllib.parse import urlparse
from .env import get_env
from .interface import Backend
from .patch import Patch
from .snowflake import Snowflake

from enum import Enum
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class SourceType(Enum):
    UNKNOWN = 0
    PATCH_GRAPHQL = 1
    SNOWFLAKE = 2


def get_source_type(source: str) -> SourceType:
    """
    Returns the source type of the input source url.

    Args:
        source: A valid URL string pointing to the source of the data resource.

    Returns:
        A source type, if supported; unknown if not supported.
    """
    url = urlparse(source)

    if url.hostname == "api.patch.tech" and url.path == "/query/graphql":
        return SourceType.PATCH_GRAPHQL
    elif url.hostname.endswith("snowflakecomputing.com"):
        return SourceType.SNOWFLAKE

    return SourceType.UNKNOWN


def make_backend(query) -> Optional[Backend]:
    """
    Makes an instance of the backend that can communicate with the source that
    holds the table's data.

    Args:
        query: Table expression that can be executed against the created backend.

    Returns:
        A Backend instance or None if the source is not supported.
    """
    version = query.dataset_version
    name = query.name
    source = query.source

    if not source:
        raise ValueError("Cannot get execution backend for query with unknown source")

    source_type = get_source_type(source)

    if source_type == SourceType.PATCH_GRAPHQL:
        auth_token = get_env("PATCH_AUTH_TOKEN")
        return Patch(source, name, version, auth_token)
    elif source_type == SourceType.SNOWFLAKE:
        dpm_agent_host = get_env('DPM_AGENT_HOST', 'localhost')
        dpm_agent_port = get_env('DPM_AGENT_PORT', '50051')
        snowflake_account = get_env('SNOWSQL_ACCOUNT')
        snowflake_user = get_env('SNOWSQL_USER')
        snowflake_password = get_env('SNOWSQL_PWD')
        snowflake_database = get_env('SNOWSQL_DATABASE')
        snowflake_schema = get_env('SNOWSQL_SCHEMA')
        return Snowflake(
            f"{dpm_agent_host}:{dpm_agent_port}",
            snowflake_account,
            snowflake_user,
            snowflake_password,
            snowflake_database,
            snowflake_schema,
        )
    else:
        logger.error(f'Unknown source type, "{source_type}", for query\'s table source "{source}"')
