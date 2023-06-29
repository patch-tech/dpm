import os


def get_env(name: str, default_value: str = None) -> str:
    """
    Returns the value of the environment variable `name`, if found. Else, returns
    `default_value` if provided, undefined otherwise.

    Args:
        name: Name of environment variable.
        default_value: Default value to return if the environment variable is not found.

    Returns:
        Value of environment variable, else specified default_value or undefined.
    """
    value = os.environ.get(name)
    if value is None:
        if default_value is not None:
            return default_value
        else:
            raise ValueError(f"Undefined env variable: {name}")
    return value
