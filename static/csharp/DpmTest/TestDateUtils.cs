using Dpm;

namespace DpmTest
{
  [TestClass]
  public class TestDateUtils
  {
    [TestMethod]
    public void TestAddDurationDateOnly()
    {
      Assert.AreEqual(
        DateUtils.AddDuration(new DateOnly(2023, 10, 12), -1, DateGranularity.years),
        new DateOnly(2022, 10, 12)
      );
      Assert.AreEqual(
        DateUtils.AddDuration(new DateOnly(2023, 2, 15), 13, DateGranularity.days),
        new DateOnly(2023, 2, 28)
      );
      Assert.AreEqual(
        DateUtils.AddDuration(new DateOnly(2023, 2, 15), 2, DateGranularity.weeks),
        new DateOnly(2023, 3, 1)
      );
    }

    [TestMethod]
    public void TestAddDurationTimeOnly()
    {
      // Clamps to zero.
      Assert.AreEqual(
        DateUtils.AddDuration(new TimeOnly(15, 2, 45), -16, TimeGranularity.hours),
        new TimeOnly(0, 0, 0)
      );

      // Clamps to last time of day.
      Assert.AreEqual(
        DateUtils.AddDuration(new TimeOnly(15, 2, 45), 9, TimeGranularity.hours),
        new TimeOnly(23, 59, 59, 999)
      );

      Assert.AreEqual(
        DateUtils.AddDuration(new TimeOnly(15, 2, 45), -12, TimeGranularity.minutes),
        new TimeOnly(14, 50, 45)
      );
    }

    [TestMethod]
    public void TestAddDurationDateTime()
    {
      Assert.AreEqual(
        DateUtils.AddDuration(new DateTime(2023, 2, 15, 15, 2, 45), -1, DateTimeGranularity.years),
        new DateTime(2022, 2, 15, 15, 2, 45)
      );

      Assert.AreEqual(
        DateUtils.AddDuration(new DateTime(2023, 2, 15, 15, 2, 45), 13, DateTimeGranularity.days),
        new DateTime(2023, 2, 28, 15, 2, 45)
      );

      Assert.AreEqual(
        DateUtils.AddDuration(new DateTime(2023, 2, 15, 15, 2, 45), 2, DateTimeGranularity.weeks),
        new DateTime(2023, 3, 1, 15, 2, 45)
      );

      Assert.AreEqual(
        DateUtils.AddDuration(new DateTime(2023, 2, 15, 15, 2, 45), -16, DateTimeGranularity.hours),
        new DateTime(2023, 2, 14, 23, 2, 45)
      );
    }
  }
}