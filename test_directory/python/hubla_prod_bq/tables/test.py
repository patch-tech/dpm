

from typing import Literal

from ..field import StringField
from ..field_expr import FieldExpr
from ..table import Table


class Test:
    # Source path.
    source_path = "https://api.patch.tech/query/graphql"

    class Map(dict):
        __getattr__ = dict.get

    # Fields.
    fields = Map({
    "pmcm_custom_id": StringField("pmcm_custom_id")
    })

    # Singleton.
    instance = None
    table_ = None

    def __init__(self):
        self.table_ = Table(
            dataset_name="hubla-prod-bq",
            dataset_version="0.0.1",
            name="test",
            source="https://api.patch.tech/query/graphql",
            fields=list(Test.fields.values())
        )
    
    @classmethod
    def get(cls) -> "Test":
        if not Test.instance:
            Test.instance = Test()
        return Test.instance

    @classmethod
    def table(cls) -> Table:
        return Test.get().table_

    @classmethod
    def select(cls, *selection: Literal["pmcm_custom_id"] | FieldExpr) -> Table:
        return Test.table().select(*selection)
