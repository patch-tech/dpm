from datetime import date
import pytest
import re

from test_snowflake import UkRealEstateRecords

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
            (county == "CAMBRIDGESHIRE") & (city == "HUNTINGDON") & date_of_transfer.before(date(2017, 1, 1))
        )
        .order_by([date_of_transfer, "DESC"])
        .limit(3)
        .execute()
    )

    assert compiled_results == '''WITH t0 AS (
  SELECT
    t2."TRANSACTON_ID" AS "TRANSACTON_ID",
    t2."PRICE" AS "PRICE",
    t2."DATE_OF_TRANSFER" AS "DATE_OF_TRANSFER",
    t2."PROPERTY_TYPE" AS "PROPERTY_TYPE",
    t2."OLD_NEW" AS "OLD_NEW",
    t2."DURATION" AS "DURATION",
    t2."CITY" AS "CITY",
    t2."DISTRICT" AS "DISTRICT",
    t2."COUNTY" AS "COUNTY",
    t2."PPD_CATEGORY_TYPE" AS "PPD_CATEGORY_TYPE",
    t2."RECORD_STATUS" AS "RECORD_STATUS"
  FROM "DEMO_DB"."PUBLIC"."UK_REAL_ESTATE_RECORDS" AS t2
  WHERE
    t2."COUNTY" = 'CAMBRIDGESHIRE' AND t2."DATE_OF_TRANSFER" < '2017-01-01'
)
SELECT
  t1."COUNTY",
  t1."CITY",
  t1."DATE_OF_TRANSFER"
FROM (
  SELECT
    t0."TRANSACTON_ID" AS "TRANSACTON_ID",
    t0."PRICE" AS "PRICE",
    t0."DATE_OF_TRANSFER" AS "DATE_OF_TRANSFER",
    t0."PROPERTY_TYPE" AS "PROPERTY_TYPE",
    t0."OLD_NEW" AS "OLD_NEW",
    t0."DURATION" AS "DURATION",
    t0."CITY" AS "CITY",
    t0."DISTRICT" AS "DISTRICT",
    t0."COUNTY" AS "COUNTY",
    t0."PPD_CATEGORY_TYPE" AS "PPD_CATEGORY_TYPE",
    t0."RECORD_STATUS" AS "RECORD_STATUS"
  FROM t0
  ORDER BY
    t0."DATE_OF_TRANSFER" DESC
) AS t1
LIMIT 3'''

    assert len(executed_results) == 3
    assert executed_results[0]['COUNTY'] == 'CAMBRIDGESHIRE'
    assert executed_results[0]['CITY'] == 'HUNTINGDON'
    assert re.match(r'^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{6}$', executed_results[0]['DATE_OF_TRANSFER'])