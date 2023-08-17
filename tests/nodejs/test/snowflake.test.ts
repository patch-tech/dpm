import { UkRealEstateRecords } from 'test-snowflake';

// The jest default timeout value is 5000ms, which is bumped to 15000ms here to give snowflake more time to return
const TIMEOUT_VALUE = 15000;

describe('UkRealEstateRecords', () => {

    it('compiles query', async () => {
        let { county, city, dateOfTransfer } = UkRealEstateRecords.fields;

        let compiledResults = await UkRealEstateRecords.select(
            county,
            city,
            dateOfTransfer
        )
            .filter(
                county
                    .eq('CAMBRIDGESHIRE')
                    .and(dateOfTransfer.before(new Date('2017-01-01')))
            )
            .orderBy([dateOfTransfer, 'DESC'])
            .limit(3)
            .compile();
        expect(compiledResults).toBe(
            `WITH t0 AS (
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
LIMIT 3`);
    });


    it('executes query', async () => {
        interface ReturnData {
            COUNTY: string,
            CITY: string,
            DATE_OF_TRANSFER: string
        }
        let { county, city, dateOfTransfer } = UkRealEstateRecords.fields;

        let executedResults = await UkRealEstateRecords.select(
            county,
            city,
            dateOfTransfer
        )
            .filter(
                county
                    .eq('CAMBRIDGESHIRE')
                    .and(city.eq('HUNTINGDON'))
                    .and(dateOfTransfer.before(new Date('2017-01-01')))
            )
            .orderBy([dateOfTransfer, 'DESC'])
            .limit(3)
            .execute();
        const timestamp = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{6}$/;
        const firstValue = executedResults[0] as ReturnData

        expect(firstValue.COUNTY).toStrictEqual("CAMBRIDGESHIRE");
        expect(firstValue.CITY).toStrictEqual("HUNTINGDON");
        expect(firstValue.DATE_OF_TRANSFER).toMatch(timestamp);
    }, TIMEOUT_VALUE);
});