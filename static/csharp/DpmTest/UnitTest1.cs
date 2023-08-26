using Dpm;

namespace DpmTest
{
    [TestClass]
    public class UnitTest1
    {
        [TestMethod]
        public void TestAuthorizationRequired()
        {
            using var client = DatasetClient.NewBuilder()
                .SetPackageId("myPackageId")
                .Build();
            var query = new DpmAgent.Query
            {
                SelectFrom = "MyTable"
            };
            try {
                client.ExecuteQuery(query);
                Assert.Fail("Query should fail due to missing authorization token");
            }
            catch (Grpc.Core.RpcException ex)
            {
                Assert.AreEqual(ex.Status.StatusCode, Grpc.Core.StatusCode.Unauthenticated);
            }
        }
    }
}