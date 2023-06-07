import os


def get_env(name: str, default_value: str = None) -> str:
    value = os.environ.get(name)
    if value is None:
        if default_value is not None:
            return default_value
        else:
            raise ValueError(f"Undefined env variable: {name}")
    return value
