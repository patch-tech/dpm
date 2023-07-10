import { UkRealEstateRecords } from 'test-patch';

describe('UkRealEstateRecords', () => {

    it('verify compiled query', async () => {
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
            `ukRealEstateRecordsQuery(filter: {
      and: [{
    county: {
      eq: "CAMBRIDGESHIRE"
    }
  },
{
    dateOfTransfer: {
      before: "2017-01-01"
    }
  }]
    }, orderBy: [{dateOfTransfer: desc}], limit: 3) {
      county
city
dateOfTransfer
    }`);
    });


    it('verify data recieved from patch', async () => {
        let { county, city, dateOfTransfer } = UkRealEstateRecords.fields;

        let executedResults = await UkRealEstateRecords.select(
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
            .execute();
        expect(executedResults).toStrictEqual([{
            city: "HUNTINGDON",
            county: "CAMBRIDGESHIRE",
            dateOfTransfer: "2016-12-31"
        },
        {
            city: "CAMBRIDGE",
            county: "CAMBRIDGESHIRE",
            dateOfTransfer: "2016-12-30"
        },
        {
            city: "CAMBRIDGE",
            county: "CAMBRIDGESHIRE",
            dateOfTransfer: "2016-12-29"
        }]);
    });
});
