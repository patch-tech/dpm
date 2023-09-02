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
    /// Returns a boolean expression that checks if this DateTime is before 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this DateTime is before 'd'.</returns>
    public BinaryBooleanFieldExpr Before(DateTime d)
    {
      // DateTime.ToString("O") returns an ISO 8601 formatted DateTime.
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.lt, new LiteralField<string>(d.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if this DateTime is after 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this DateTime is after 'd'</returns>
    public BinaryBooleanFieldExpr After(DateTime d)
    {
      // DateTime.ToString("O") returns an ISO 8601 formatted DateTime.
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.gt, new LiteralField<string>(d.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is before DateTime 'b'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is before 'b'</returns>
    public static BinaryBooleanFieldExpr operator <(DateTimeField a, DateTime b)
    {
      return a.Before(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is after DateTime 'b'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is after 'b'</returns>
    public static BinaryBooleanFieldExpr operator >(DateTimeField a, DateTime b)
    {
      return a.After(b);
    }

    /// <summary>
    /// Returns a boolean expression that performs a relative range check of this DateTimeField.
    /// The range is specified by its two bounds and a granularity.
    /// E.g., the filter expression below checks if the value of `startDate` lies
    /// in the past 2 to 3 hours.
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
      // TODO(PAT-3355): Generate the relative datetime ranges and use the `between` operation.
      return new BinaryBooleanFieldExpr(
        this,
        BooleanOperatorType.inPast,
        new LiteralField<int>(new int[] { olderThan_, newerThan_, (int)granularity })
      );
    }

  }

}