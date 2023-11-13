using System.Runtime.InteropServices;

namespace Dpm
{

  public class DateField : Field<DateOnly>
  {
    public DateField(string name) : base(name)
    {
    }

    /// <summary>
    /// Projects the date to its year.
    /// </summary>
    public DerivedField<int, DateOnly> Year
    {
      get
      {
        return new DerivedField<int, DateOnly>(this, ProjectionOperatorType.year);
      }
    }

    /// <summary>
    /// Projects the date to its month.
    /// </summary>
    public DerivedField<int, DateOnly> Month
    {
      get
      {
        return new DerivedField<int, DateOnly>(this, ProjectionOperatorType.month);
      }
    }

    /// <summary>
    /// Projects the date to its day.
    /// </summary>
    public DerivedField<int, DateOnly> Day
    {
      get
      {
        return new DerivedField<int, DateOnly>(this, ProjectionOperatorType.day);
      }
    }

    /// <summary>
    /// Projects the date to its day of week.
    /// </summary>
    public DerivedField<int, DateOnly> DayOfWeek
    {
      get
      {
        return new DerivedField<int, DateOnly>(this, ProjectionOperatorType.dayOfWeek);
      }
    }

    /// <summary>
    /// Projects the date to its week of year.
    /// </summary>
    public DerivedField<int, DateOnly> Week
    {
      get
      {
        return new DerivedField<int, DateOnly>(this, ProjectionOperatorType.week);
      }
    }

    // TODO(PAT-4879): Support `weekDate` projection.

    /// <summary>
    /// Returns a boolean expression that checks if this DateField is before 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this DateField is before 'd'.</returns>
    public BinaryBooleanFieldExpr Before(DateOnly d)
    {
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.lt, new LiteralField<string>(DateUtils.ToString(d)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if this DateField is after 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this DateField is after 'd'</returns>
    public BinaryBooleanFieldExpr After(DateOnly d)
    {
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.gt, new LiteralField<string>(DateUtils.ToString(d)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is before DateOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is before 'b'</returns>
    public static BinaryBooleanFieldExpr operator <(DateField a, DateOnly b)
    {
      return a.Before(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is after DateOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is after 'b'</returns>
    public static BinaryBooleanFieldExpr operator >(DateField a, DateOnly b)
    {
      return a.After(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is equal to DateOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator ==(DateField a, DateOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.eq, new LiteralField<string>(DateUtils.ToString(b)));
    }


    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is not equal to DateOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is not equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator !=(DateField a, DateOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.neq, new LiteralField<string>(DateUtils.ToString(b)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is less than or equal to DateOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is less than or equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator <=(DateField a, DateOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.lte, new LiteralField<string>(DateUtils.ToString(b)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is greater than or equal to DateOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is greater than or equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator >=(DateField a, DateOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.gte, new LiteralField<string>(DateUtils.ToString(b)));
    }

    /// <summary>
    /// Returns a boolean expression that performs a relative range check of this DateField.
    /// The range is specified by its two bounds and a granularity.
    /// E.g., the filter expression below checks if the value of `startDate` lies
    /// in the past 2 to 3 weeks, inclusive of bounds.
    /// <c>
    /// let query = MyTable
    ///    .Select(startDate, name)
    ///    .Filter(startDate.InPast(2, 3, DateGranularity.weeks))
    /// </c>
    /// </summary>
    public BinaryBooleanFieldExpr InPast(int olderThan, int newerThan, DateGranularity granularity)
    {
      var (olderThan_, newerThan_) = (olderThan, newerThan);
      if (olderThan > newerThan)
      {
        Console.WriteLine(
          $"InPast specified with olderThan({olderThan}) > newerThan(${newerThan}), swapped arguments."
        );
        (olderThan_, newerThan_) = (newerThan, olderThan);
      }
      var today = DateOnly.FromDateTime(DateTime.UtcNow);
      var upperBound = DateUtils.AddDuration(today, -olderThan_, granularity);
      var lowerBound = DateUtils.AddDuration(today, -newerThan_, granularity);

      return this >= lowerBound & this <= upperBound;
    }
  }

  public class DateUtils
  {
    public static DateOnly AddDuration(DateOnly d, int n, DateGranularity granularity)
    {
      return granularity switch
      {
        DateGranularity.years => d.AddYears(n),
        DateGranularity.months => d.AddMonths(n),
        DateGranularity.weeks => d.AddDays(7 * n),
        DateGranularity.days => d.AddDays(n),
        _ => throw new Exception($"Unknown DateGranularity {Enum.GetName(typeof(DateGranularity), granularity)}")
      };
    }

    public static TimeOnly AddDuration(TimeOnly t, int n, TimeGranularity granularity)
    {
      int wrap;
      var result = granularity switch
      {
        TimeGranularity.hours => t.AddHours(n, out wrap),
        TimeGranularity.minutes => t.AddMinutes(n, out wrap),
        TimeGranularity.seconds => t.AddMinutes(n / 60.0, out wrap),
        TimeGranularity.milliseconds => t.AddMinutes(n / 60_000.0, out wrap),
        _ => throw new Exception($"Unknown TimeGranularity {Enum.GetName(typeof(TimeGranularity), granularity)}")
      };

      if (wrap == 0)
      {
        return result;
      }


      if (wrap > 0)
      {
        // Wrapped ahead, clamp to EOD.
        return new TimeOnly(23, 59, 59, 999);
      }

      // Wrapped below, clamp to 0.
      return new TimeOnly(0, 0, 0, 0);
    }

    public static DateTime AddDuration(DateTime dt, int n, DateTimeGranularity granularity)
    {
      return granularity switch
      {
        DateTimeGranularity.years => dt.AddYears(n),
        DateTimeGranularity.months => dt.AddMonths(n),
        DateTimeGranularity.weeks => dt.AddDays(7 * n),
        DateTimeGranularity.days => dt.AddDays(n),
        DateTimeGranularity.hours => dt.AddHours(n),
        DateTimeGranularity.minutes => dt.AddMinutes(n),
        DateTimeGranularity.seconds => dt.AddMinutes(n / 60.0),
        DateTimeGranularity.milliseconds => dt.AddMinutes(n / 60_000.0),
        _ => throw new Exception($"Unknown DateTimeGranularity {Enum.GetName(typeof(DateTimeGranularity), granularity)}")
      };
    }

    public static string ToString(DateOnly d)
    {
      // DateOnly.ToString("O") returns an ISO 8601 formatted date.
      return d.ToString("O");
    }

    public static string ToString(TimeOnly t)
    {
      return t.ToString("HH:mm:ss.fff");
    }

    public static string ToString(DateTime dt)
    {
      // Return ISO 8601 formatted DateTime in UTC without the offset.
      // NB: `.ToString("O")` includes a time offset from UTC, which we do not
      // want in the output, so we specify a format string.
      // See: https://learn.microsoft.com/en-us/dotnet/standard/base-types/custom-date-and-time-format-strings#fffSpecifier
      return dt.ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ");
    }
  }
}
