import asyncio
from datetime import date
from pprint import pprint

from test_patch import UkRealEstateRecords

async def main():
    county, city, date_of_transfer = [
        UkRealEstateRecords.fields.county,
        UkRealEstateRecords.fields.city,
        UkRealEstateRecords.fields.date_of_transfer,
    ]

    results = (
        await UkRealEstateRecords.select(county, city, date_of_transfer)
        .filter(
            (county == "CAMBRIDGESHIRE") & date_of_transfer.before(date(2017, 1, 1))
        )
        .order_by([date_of_transfer, "DESC"])
        .limit(3)
        .execute()
    )

    pprint(results)


asyncio.run(main())
