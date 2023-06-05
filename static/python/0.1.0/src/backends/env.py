import os

def getEnv(name: str, defaultValue: str = None) -> str:
    value = os.environ.get(name)
    if value is None:
        if defaultValue is not None:
            return defaultValue
        else:
            raise ValueError(f"Undefined env variable: {name}")
    return value
