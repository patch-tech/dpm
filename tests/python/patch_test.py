from datetime import date
import pytest

from test_patch import UkRealEstateRecords

@pytest.mark.asyncio
async def test_results():
    county, city, date_of_transfer = [
        UkRealEstateRecords.fields.county,
        UkRealEstateRecords.fields.city,
        UkRealEstateRecords.fields.date_of_transfer,
    ]

    compiled_results = (
        await UkRealEstateRecords.select(county, city, date_of_transfer)
        .filter(
            (county == "CAMBRIDGESHIRE") & date_of_transfer.before(date(2017, 1, 1))
        )
        .order_by([date_of_transfer, "DESC"])
        .limit(3)
        .compile()
    )
    executed_results = (
        await UkRealEstateRecords.select(county, city, date_of_transfer)
        .filter(
            (county == "CAMBRIDGESHIRE") & date_of_transfer.before(date(2017, 1, 1))
        )
        .order_by([date_of_transfer, "DESC"])
        .limit(3)
        .execute()
    )
    assert compiled_results == ('ukRealEstateRecordsQuery(filter: {\n'
                                '      and: [{\n'
                                '    county: {\n'
                                '      eq: "CAMBRIDGESHIRE"\n'
                                '    }\n'
                                '  },\n'
                                '{\n'
                                '    dateOfTransfer: {\n'
                                '      before: "2017-01-01"\n'
                                '    }\n'
                                '  }]\n'
                                '    }, orderBy: [{dateOfTransfer: desc}], limit: 3) {\n'
                                'county\n'
                                'city\n'
                                'dateOfTransfer\n'
                                '}')
    assert executed_results == [{'city': 'HUNTINGDON', 
                                 'county': 'CAMBRIDGESHIRE',
                                 'dateOfTransfer': '2016-12-31'},
                                {'city': 'CAMBRIDGE',
                                 'county': 'CAMBRIDGESHIRE',
                                 'dateOfTransfer': '2016-12-30'},
                                {'city': 'CAMBRIDGE',
                                 'county': 'CAMBRIDGESHIRE',
                                 'dateOfTransfer': '2016-12-29'}]
    