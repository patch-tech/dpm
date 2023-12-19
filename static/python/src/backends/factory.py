"""Factory to create an execution backend instance."""
import json
import logging
import os
import platform
from typing import Optional

from ..backends.dpm_agent.dpm_agent_client import make_client
from .env import get_env
from .interface import Backend


logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


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
        root_dir = os.path.expanduser("~")
        session_path = ""
        if platform.system() == "Darwin":
            session_path = os.path.join(
                root_dir,
                "Library",
                "Application Support",
                "tech.patch.dpm",
                "session.json",
            )
        elif platform.system() == "Windows":
            session_path = os.path.join(
                root_dir, "AppData", "Roaming", "patch", "session.json"
            )
        elif platform.system() == "Linux":
            session_path = os.path.join(root_dir, ".config", "dpm", "session.json")

        try:
            with open(session_path, "r") as f:
                session_data = json.load(f)
                return session_data.get("access_token", None)
        except Exception as e:
            print(f"Error receiving access token from project directory: {e}")
            return None


def make_backend() -> Backend:
    """
    Makes an instance of the backend that can communicate with `dpm-agent` to
    compile and execute queries.

    Returns:
        A Backend instance.
    """
    dpm_auth_token = get_dpm_auth_token()
    if not dpm_auth_token:
        raise ValueError(
            "Failed to find DPM authentication token. Please run `dpm login`"
        )

    dpm_agent_address = get_env("DPM_AGENT_URL", "https://agent.dpm.sh")
    return make_client(
        dpm_agent_address,
        dpm_auth_token,
    )
