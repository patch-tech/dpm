using Google.Protobuf;
using DpmAgent;

namespace Dpm
{
  public class ArrayField<T> : Field<T>
  {
    public ArrayField(string name) : base(name)
    {
    }

    /// <summary>
    /// Returns a boolean expression that checks if the field has any of the values present in `vals`.
    /// </summary>
    public BinaryBooleanFieldExpr HasAny(T[] vals)
    {
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.hasAny, new LiteralField<T>(vals));
    }

    /// <summary>
    /// Returns a boolean expression that checks if the field has all the values present in `vals`.
    /// </summary>
    public BinaryBooleanFieldExpr HasAll(T[] vals)
    {
      return new BinaryBooleanFieldExpr(this, BooleanOperatorType.hasAll, new LiteralField<T>(vals));
    }

    public new AggregateFieldExpr<T> Max()
    {
      throw new NotImplementedException("Cannot call Max on ArrayField");
    }

    public new AggregateFieldExpr<T> Min()
    {
      throw new NotImplementedException("Cannot call Min on ArrayField");
    }

    public new AggregateFieldExpr<T> Sum()
    {
      throw new NotImplementedException("Cannot call Sum on ArrayField");
    }

    public new AggregateFieldExpr<T> Count()
    {
      throw new NotImplementedException("Cannot call Count on ArrayField");
    }

    public new AggregateFieldExpr<T> CountDistinct()
    {
      throw new NotImplementedException("Cannot call CountDistinct on ArrayField");
    }

    public new AggregateFieldExpr<T> Avg()
    {
      throw new NotImplementedException("Cannot call Avg on ArrayField");
    }

    public new AggregateFieldExpr<T> AvgDistinct()
    {
      throw new NotImplementedException("Cannot call AvgDistinct on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator ==(ArrayField<T> a, T b)
    {
      throw new NotImplementedException("Cannot call == on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator ==(ArrayField<T> a, ArrayField<T> b)
    {
      throw new NotImplementedException("Cannot call == on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator !=(ArrayField<T> a, T b)
    {
      throw new NotImplementedException("Cannot call != on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator !=(ArrayField<T> a, ArrayField<T> b)
    {
      throw new NotImplementedException("Cannot call != on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator >(ArrayField<T> a, T b)
    {
      throw new NotImplementedException("Cannot call > on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator >(ArrayField<T> a, ArrayField<T> b)
    {
      throw new NotImplementedException("Cannot call > on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator >=(ArrayField<T> a, T b)
    {
      throw new NotImplementedException("Cannot call >= on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator >=(ArrayField<T> a, ArrayField<T> b)
    {
      throw new NotImplementedException("Cannot call >= on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator <(ArrayField<T> a, T b)
    {
      throw new NotImplementedException("Cannot call < on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator <(ArrayField<T> a, ArrayField<T> b)
    {
      throw new NotImplementedException("Cannot call < on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator <=(ArrayField<T> a, T b)
    {
      throw new NotImplementedException("Cannot call <= on ArrayField");
    }

    public static BinaryBooleanFieldExpr operator <=(ArrayField<T> a, ArrayField<T> b)
    {
      throw new NotImplementedException("Cannot call <= on ArrayField");
    }

    public new BinaryBooleanFieldExpr In(T[] that)
    {
      throw new NotImplementedException("Cannot call In on ArrayField");
    }

    public new BinaryBooleanFieldExpr Between(T minVal, T maxVal)
    {
      throw new NotImplementedException("Cannot call Between on ArrayField");
    }
  }
}