namespace Dpm
{

  public class TimeField : Field<TimeOnly>
  {
    public TimeField(string name) : base(name)
    {
    }

    /// <summary>
    /// Projects the Time to its hour.
    /// </summary>
    public DerivedField<int, TimeOnly> Hour
    {
      get
      {
        return new DerivedField<int, TimeOnly>(this, ProjectionOperatorType.hour);
      }
    }

    /// <summary>
    /// Projects the Time to its minute.
    /// </summary>
    public DerivedField<int, TimeOnly> Minute
    {
      get
      {
        return new DerivedField<int, TimeOnly>(this, ProjectionOperatorType.minute);
      }
    }

    /// <summary>
    /// Projects the Time to its second.
    /// </summary>
    public DerivedField<int, TimeOnly> Second
    {
      get
      {
        return new DerivedField<int, TimeOnly>(this, ProjectionOperatorType.second);
      }
    }

    /// <summary>
    /// Returns a boolean expression that checks if this TimeField is before 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this TimeField is before 'd'.</returns>
    public BinaryBooleanFieldExpr Before(TimeOnly d)
    {
      // TimeOnly.ToString("O") returns an ISO 8601 formatted time.
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.lt, new LiteralField<string>(d.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if this TimeField is after 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this TimeField is after 'd'</returns>
    public BinaryBooleanFieldExpr After(TimeOnly d)
    {
      // TimeOnly.ToString("O") returns an ISO 8601 formatted time.
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.gt, new LiteralField<string>(d.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if TimeField 'a' is before TimeOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if TimeField 'a' is before 'b'</returns>
    public static BinaryBooleanFieldExpr operator <(TimeField a, TimeOnly b)
    {
      return a.Before(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if TimeField 'a' is after TimeOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if TimeField 'a' is after 'b'</returns>
    public static BinaryBooleanFieldExpr operator >(TimeField a, TimeOnly b)
    {
      return a.After(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if TimeField 'a' is equal to TimeOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if TimeField 'a' is equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator ==(TimeField a, TimeOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.eq, new LiteralField<string>(b.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if TimeField 'a' is not equal to TimeOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if TimeField 'a' is not equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator !=(TimeField a, TimeOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.neq, new LiteralField<string>(b.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if TimeField 'a' is less than or equal to TimeOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if TimeField 'a' is less than or equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator <=(TimeField a, TimeOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.lte, new LiteralField<string>(b.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if TimeField 'a' is greater than or equal to TimeOnly 'b'.
    /// </summary>
    /// <param name="a"></param>
    /// <param name="b"></param>
    /// <returns>A boolean expression that checks if TimeField 'a' is greater than or equal to 'b'</returns>
    public static BinaryBooleanFieldExpr operator >=(TimeField a, TimeOnly b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.gte, new LiteralField<string>(b.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that performs a relative range check of this TimeField.
    /// The range is specified by its two bounds and a granularity.
    /// E.g., the filter expression below checks if the value of `startDate` lies
    /// in the past 2 to 3 hours, inclusive of bounds.
    /// <c>
    /// let query = MyTable
    ///    .Select(startDateTime, name)
    ///    .Filter(startDateTime.InPast(2, 3, TimeGranularity.hours))
    /// </c>
    /// </summary>
    public BinaryBooleanFieldExpr InPast(int olderThan, int newerThan, TimeGranularity granularity)
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