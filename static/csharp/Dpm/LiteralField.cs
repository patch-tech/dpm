namespace Dpm
{
  public class LiteralField<T> : Field<T>
  {
    public Scalar<T> Value;

    public LiteralField(T value_) : base($"lit")
    {
      Value = new Scalar<T>(value_);
    }

    public LiteralField(T[] value_) : base($"lit")
    {
      Value = new Scalar<T>(value_);
    }

    public new AggregateFieldExpr<T> Max() {
        throw new NotImplementedException("Cannot call Max on LiteralField");
    }

    public new AggregateFieldExpr<T> Min() {
        throw new NotImplementedException("Cannot call Min on LiteralField");
    }

    public new AggregateFieldExpr<T> Count() {
        throw new NotImplementedException("Cannot call Count on LiteralField");
    }

    public new AggregateFieldExpr<T> CountDistinct() {
        throw new NotImplementedException("Cannot call CountDistinct on LiteralField");
    }

    public new AggregateFieldExpr<T> Avg() {
        throw new NotImplementedException("Cannot call Avg on LiteralField");
    }

    public new AggregateFieldExpr<T> AvgDistinct() {
        throw new NotImplementedException("Cannot call AvgDistinct on LiteralField");
    }
  }
}