namespace Dpm
{

  public class DateTimeField : Field<DateTime>
  {
    public DateTimeField(string name) : base(name)
    {
    }

    /// <summary>
    /// Projects the DateTime to its year.
    /// </summary>
    public DerivedField<int, DateTime> Year
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.year);
      }
    }


    /// <summary>
    /// Projects the DateTime to its month.
    /// </summary>
    public DerivedField<int, DateTime> Month
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.month);
      }
    }

    /// <summary>
    /// Projects the DateTime to its day.
    /// </summary>
    public DerivedField<int, DateTime> Day
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.day);
      }
    }

    /// <summary>
    /// Projects the DateTime to its day of week.
    /// </summary>
    public DerivedField<int, DateTime> DayOfWeek
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.dayOfWeek);
      }
    }

    /// <summary>
    /// Projects the DateTime to its week of year.
    /// </summary>
    public DerivedField<int, DateTime> Week
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.week);
      }
    }

    // TODO(PAT-4879): Support `weekDate` projection.

    /// <summary>
    /// Projects the DateTime to its date.
    /// </summary>
    public DerivedField<int, DateTime> Date
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.date);
      }
    }

    /// <summary>
    /// Projects the DateTime to its time.
    /// </summary>
    public DerivedField<int, DateTime> Time
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.time);
      }
    }

    /// <summary>
    /// Projects the DateTime to its hour.
    /// </summary>
    public DerivedField<int, DateTime> Hour
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.hour);
      }
    }

    /// <summary>
    /// Projects the DateTime to its minute.
    /// </summary>
    public DerivedField<int, DateTime> Minute
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.minute);
      }
    }

    /// <summary>
    /// Projects the DateTime to its second.
    /// </summary>
    public DerivedField<int, DateTime> Second
    {
      get
      {
        return new DerivedField<int, DateTime>(this, ProjectionOperatorType.second);
      }
    }

    /// <summary>
    /// Returns a boolean expression that checks if this DateTimeField is before 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this DateTimeField is before 'd'.</returns>
    public BinaryBooleanFieldExpr Before(DateTime d)
    {
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.lt, new LiteralField<string>(DateUtils.ToString(d)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if this DateTimeField is after 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this DateTimeField is after 'd'</returns>
    public BinaryBooleanFieldExpr After(DateTime d)
    {
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.gt, new LiteralField<string>(DateUtils.ToString(d)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateTimeField 'a' is before DateTime 'b'.
    /// </summary>
    /// <param name="d"></param>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateTimeField 'a' is before 'b'</returns>
    public static BinaryBooleanFieldExpr operator <(DateTimeField a, DateTime b)
    {
      return a.Before(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateTimeField 'a' is after DateTime 'b'.
    /// </summary>
    /// <param name="d"></param>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateTimeField 'a' is after 'b'</returns>
    public static BinaryBooleanFieldExpr operator >(DateTimeField a, DateTime b)
    {
      return a.After(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateTimeField 'a' is equal to DateTime 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>

    /// <returns>A boolean expression that checks if DateTimeField 'a' is equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator ==(DateTimeField a, DateTime b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.eq, new LiteralField<string>(DateUtils.ToString(b)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateTimeField 'a' is not equal to DateTime 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateTimeField 'a' is not equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator !=(DateTimeField a, DateTime b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.neq, new LiteralField<string>(DateUtils.ToString(b)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateTimeField 'a' is less than or equal to DateTime 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateTimeField 'a' is less than or equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator <=(DateTimeField a, DateTime b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.lte, new LiteralField<string>(DateUtils.ToString(b)));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateTimeField 'a' is greater than or equal to DateTime 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if DateTimeField 'a' is greater than or equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator >=(DateTimeField a, DateTime b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.gte, new LiteralField<string>(DateUtils.ToString(b)));
    }

    /// <summary>
    /// Returns a boolean expression that performs a relative range check of this DateTimeField.
    /// The range is specified by its two bounds and a granularity.
    /// E.g., the filter expression below checks if the value of `startDate` lies
    /// in the past 2 to 3 hours, inclusive of bounds.
    /// <c>
    /// let query = MyTable
    ///    .Select(startDateTime, name)
    ///    .Filter(startDateTime.InPast(2, 3, DateGranularity.hours))
    /// </c>
    /// </summary>
    public BinaryBooleanFieldExpr InPast(int olderThan, int newerThan, DateTimeGranularity granularity)
    {
      var (olderThan_, newerThan_) = (olderThan, newerThan);
      if (olderThan > newerThan)
      {
        Console.WriteLine(
          $"InPast specified with olderThan({olderThan}) > newerThan(${newerThan}), swapped arguments."
        );
        (olderThan_, newerThan_) = (newerThan, olderThan);
      }

      var now = DateTime.UtcNow;
      var upperBound = DateUtils.AddDuration(now, -olderThan_, granularity);
      var lowerBound = DateUtils.AddDuration(now, -newerThan_, granularity);

      return this >= lowerBound & this <= upperBound;
    }
  }
}
