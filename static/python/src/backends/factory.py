"""Factory to create the execution backend instance based on the query table's source."""
import json
import logging
import os
import platform
from enum import Enum
from typing import Optional
from urllib.parse import urlparse

from .env import get_env
from .interface import Backend
from .patch import Patch
from .snowflake import Snowflake

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


def get_dpm_auth_token() -> Optional[str]:
    """
    Discovers the `dpm` authentication token by inspecting:
    1. Environment variable DPM_AUTH_TOKEN
    2. The session.json file stored by `dpm login`.
    3. ...
    """
    try:
        dpm_auth_token = get_env("DPM_AUTH_TOKEN")
        if dpm_auth_token:
            return dpm_auth_token

    except:
        root_dir = os.path.expanduser('~')
        session_path = ''
        if platform.system() == 'Darwin':
            session_path = os.path.join(root_dir, 'Library', 'Application Support', 'tech.patch.dpm', 'session.json')
        elif platform.system() == 'Windows':
            session_path = os.path.join(root_dir, 'AppData', 'Roaming', 'patch', 'session.json')
        elif platform.system() == 'Linux':
            session_path = os.path.join(root_dir, '.config', 'dpm', 'session.json')
        
        try:
            with open(session_path, 'r') as f:
                session_data = json.load(f)
                return session_data.get('access_token', None)
        except Exception as e:
            print(f"Error receiving access token from project directory: {e}")
            return None


def make_backend(query) -> Optional[Backend]:
    """
    Makes an instance of the backend that can communicate with the source that
    holds the table's data.

    Args:
        query: Table expression that can be executed against the created backend.

    Returns:
        A Backend instance or None if the source is not supported.
    """
    dataset_version = query.dataset_version
    name = query.name
    source = query.source

    if not source:
        raise ValueError("Cannot get execution backend for query with unknown source")

    source_type = get_source_type(source)
    dpm_auth_token = get_dpm_auth_token()
    if not dpm_auth_token:
        raise ValueError(
            "Failed to find DPM authentication token. Please run `dpm login`"
        )

    if source_type == SourceType.PATCH_GRAPHQL:
        auth_token = get_env("PATCH_AUTH_TOKEN")
        return Patch(source, name, dataset_version, auth_token)
    elif source_type == SourceType.SNOWFLAKE:
        dpm_agent_url = get_env("DPM_AGENT_URL", "https://agent.dpm.sh")
        snowflake_account = get_env("SNOWSQL_ACCOUNT")
        snowflake_user = get_env("SNOWSQL_USER")
        snowflake_password = get_env("SNOWSQL_PWD")
        snowflake_database = get_env("SNOWSQL_DATABASE")
        snowflake_schema = get_env("SNOWSQL_SCHEMA")
        return Snowflake(
            dpm_agent_url,
            dpm_auth_token,
            snowflake_account,
            snowflake_user,
            snowflake_password,
            snowflake_database,
            snowflake_schema,
            dataset_version,
        )
    else:
        logger.error(
            f'Unknown source type, "{source_type}", for query\'s table source "{source}"'
        )
