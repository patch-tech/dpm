using Dpm;
using DpmAgent;

namespace DpmTest
{
    [TestClass]
    public class UnitTest1
    {
        [TestMethod]
        public void TestAuthorizationRequired()
        {
            using var client = DpmAgentClient.NewBuilder()
                .SetPackageId("myPackageId")
                .Build();
            var query = new DpmAgent.Query
            {
                SelectFrom = "MyTable"
            };
            try
            {
                client.ExecuteQuery(query);
                Assert.Fail("Query should fail due to missing authorization token");
            }
            catch (Grpc.Core.RpcException ex)
            {
                Assert.AreEqual(ex.Status.StatusCode, Grpc.Core.StatusCode.Unauthenticated);
            }
        }

        [TestMethod]
        public void TestGetDpmAuthTokenFromEnvVar()
        {
            Environment.SetEnvironmentVariable("DPM_AUTH_TOKEN", "the-token");
            var token = Env.GetDpmAuthToken();
            Assert.AreEqual(token, "the-token");
        }

        [TestMethod]
        public void TestQuery()
        {
            DateField startedAt = new("startedAt");
            StringField name = new("name");
            Field<float> price = new("price");
            Table t = new(
                packageId: "1124-111",
                datasetName: "my_dataset",
                datasetVersion: "0.1.0",
                name: "my_table",
                fields: new FieldExpr[] { startedAt, name, price });
            Table query = t
                .Select(
                    name.As("Name"),
                    price.Max().As("Max price"),
                    startedAt.Month.As("Start month"))
                .Filter(name.Like("%ammy%") | price > 10.0f)
                .OrderBy((price.Max(), Direction.DESC))
                .Limit(10);
            var dpmQuery = DpmAgentQueryFactory.MakeQuery(query);
            var wantQueryStr = "{ \"id\": { \"packageId\": \"1124-111\" }, \"selectFrom\": \"my_table\", " +
            "\"select\": [ { \"argument\": { \"field\": { \"fieldName\": \"name\" } }, \"alias\": \"Name\" }, " +
            "{ \"argument\": { \"aggregate\": { \"op\": \"MAX\", \"argument\": { \"field\": " +
            "{ \"fieldName\": \"price\" } } } }, \"alias\": \"Max price\" }, { \"argument\": { \"derived\": " +
            "{ \"op\": \"MONTH\", \"argument\": { \"field\": { \"fieldName\": \"startedAt\" } } } }," +
            " \"alias\": \"Start month\" } ], \"filter\": { \"op\": \"OR\", \"arguments\": [ { \"condition\": { \"op\": \"LIKE\", " +
            "\"arguments\": [ { \"field\": { \"fieldName\": \"name\" } }, { \"literal\": { \"string\": \"%ammy%\" }" +
            " } ] } }, { \"condition\": { \"op\": \"GT\", \"arguments\": [ { \"field\": { \"fieldName\": \"price\" } }," +
            " { \"literal\": { \"f32\": 10 } } ] } } ] }, \"groupBy\": [ { \"field\": { \"fieldName\": \"name\" } }," +
            " { \"derived\": { \"op\": \"MONTH\", \"argument\": { \"field\": { \"fieldName\": \"startedAt\" } } } } ]," +
            " \"orderBy\": [ { \"argument\": { \"aggregate\": { \"op\": \"MAX\", \"argument\": { \"field\": " +
            "{ \"fieldName\": \"price\" } } } }, \"direction\": \"DESC\" } ], \"limit\": \"10\", \"clientVersion\": " +
            "{ \"client\": \"CSHARP\", \"datasetVersion\": \"0.1.0\", \"codeVersion\": \"0.1.0\" } }";
            Assert.AreEqual(wantQueryStr, dpmQuery.ToString());
        }
    }
}