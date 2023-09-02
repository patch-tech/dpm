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
    /// Returns a boolean expression that checks if this date is before 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this date is before 'd'.</returns>
    public BinaryBooleanFieldExpr Before(DateOnly d)
    {
      // DateOnly.ToString("O") returns an ISO 8601 formatted date.
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.lt, new LiteralField<string>(d.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if this date is after 'd'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if this date is after 'd'</returns>
    public BinaryBooleanFieldExpr After(DateOnly d)
    {
      // DateOnly.ToString("O") returns an ISO 8601 formatted date.
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.gt, new LiteralField<string>(d.ToString("O")));
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is before DateOnly 'b'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is before 'b'</returns>
    public static BinaryBooleanFieldExpr operator <(DateField a, DateOnly b)
    {
      return a.Before(b);
    }

    /// <summary>
    /// Returns a boolean expression that checks if DateField 'a' is after DateOnly 'b'.
    /// </summary>
    /// <param name="d"></param>
    /// <returns>A boolean expression that checks if DateField 'a' is after 'b'</returns>
    public static BinaryBooleanFieldExpr operator >(DateField a, DateOnly b)
    {
      return a.After(b);
    }

    /// <summary>
    /// Returns a boolean expression that performs a relative range check of this DateField.
    /// The range is specified by its two bounds and a granularity.
    /// E.g., the filter expression below checks if the value of `startDate` lies
    /// in the past 2 to 3 weeks.
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
      // TODO(PAT-3355): Generate the relative datetime ranges and use the `between` operation.
      return new BinaryBooleanFieldExpr(
        this,
        BooleanOperatorType.inPast,
        new LiteralField<int>(new int[] { olderThan_, newerThan_, (int)granularity })
      );
    }
  }
}