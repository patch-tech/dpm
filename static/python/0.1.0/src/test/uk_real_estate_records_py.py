from field import DateField, Field, StringField
from table import Table
import asyncio

# Import the dataset.
from .snowflake_ds import snowflakeDs


class UkRealEstateRecords:
    sourcePath = "https://api.patch.tech/query/graphql"

    class Map(dict):
        __getattr__ = dict.get
        __setattr__ = dict.__setitem__
        __delattr__ = dict.__delitem__

    # Fields.
    fields = Map(
        {
            "transactionId": StringField("TRANSACTION_ID"),
            "price": StringField("PRICE"),
            "dateOfTransfer": DateField("DATE_OF_TRANSFER"),
            "propertyType": StringField("PROPERTY_TYPE"),
            "oldNew": StringField("OLD_NEW"),
            "duration": StringField("DURATION"),
            "city": StringField("CITY"),
            "district": StringField("DISTRICT"),
            "county": StringField("COUNTY"),
            "ppdCategoryType": StringField("PPD_CATEGORY_TYPE"),
            "recordStatus": Field("RECORD_STATUS"),
        }
    )

    # Singleton.
    instance_ = None
    table_ = None

    def __init__(self):
        raise RuntimeError(
            "Constructor of singleton disabled. Call UkRealEstateRecords.instance()"
        )

    @classmethod
    def instance(cls):
        if cls.instance_ is None:
            cls.instance_ = cls.__new__(cls)
            cls.instance_.table_ = Table(
                snowflakeDs,
                "UK_REAL_ESTATE_RECORDS",
                cls.fields.values(),
                source="https://api.patch.tech/query/graphql",
            )
        return cls.instance_

    @classmethod
    def table(cls):
        return cls.instance().table_

    @classmethod
    def select(cls, *selection) -> Table:
        return cls.table().select(*selection)


snowflakeDs.add_table(UkRealEstateRecords.table())

county = UkRealEstateRecords.fields.county
price = UkRealEstateRecords.fields["price"]
query = (
    UkRealEstateRecords.select(county, "PRICE")
    .filter((county == "STAFFORDSHIRE") | (price == "181995"))
    .limit(10)
)
compiled = asyncio.run(query.compile())
executed = asyncio.run(query.execute())
print("QUERY:\n", compiled, "\nDATA: ", executed)
