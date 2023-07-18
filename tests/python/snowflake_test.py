from datetime import date
import pytest

from test_snowflake import UkRealEstateRecords

@pytest.mark.asyncio
async def test_results():
    county, city, date_of_transfer = [
        UkRealEstateRecords.fields.county,
        UkRealEstateRecords.fields.city,
        UkRealEstateRecords.fields.date_of_transfer,
    ]

    executed_results = (
        await UkRealEstateRecords.select(county, city, date_of_transfer)
        .filter(
            (county == "CAMBRIDGESHIRE") & date_of_transfer.before(date(2017, 1, 1))
        )
        .order_by([date_of_transfer, "DESC"])
        .limit(3)
        .execute()
    )

    assert executed_results == [{'COUNTY': 'CAMBRIDGESHIRE',
                                 'CITY': 'WISBECH',
                                 'DATE_OF_TRANSFER': '2001-12-14T00:00:00.000000'},
                                {'COUNTY': 'CAMBRIDGESHIRE',
                                 'CITY': 'CAMBRIDGE',
                                 'DATE_OF_TRANSFER': '2001-12-14T00:00:00.000000'},
                                {'COUNTY': 'CAMBRIDGESHIRE',
                                 'CITY': 'CAMBRIDGE',
                                 'DATE_OF_TRANSFER': '2001-12-14T00:00:00.000000'}]
    