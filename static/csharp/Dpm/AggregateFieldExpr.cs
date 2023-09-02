namespace Dpm
{
  /// <summary>
  /// A field expression to represent an aggregation applied on a field expression.
  /// E.g., a sum of a field can be expressed as:
  /// <c>
  /// var price = new Field<number>("price");
  /// var totalPrice = new AggregateFieldExpr<number>(price, AggregateOperatorType.sum);
  /// </c>
  /// </summary>
  public class AggregateFieldExpr<T> : FieldExpr
  {
    private readonly FieldExpr field;
    private readonly Operator.Aggregate op;

    public AggregateFieldExpr(FieldExpr field, AggregateOperatorType opType_) : base($"(${opType_}(${field.Name}))")
    {
      this.field = field;
      this.op = new Operator.Aggregate(opType_);
    }

    public override Operator Operator()
    {
      return op;
    }

    public override FieldExpr[] Operands()
    {
      return new[] { this.field };
    }

    /// <summary>
    /// Alias this expression. This method is useful when the aggregate expression
    /// is defined in a `Select` and must be referred to in a subsequent `OrderBy`.
    /// E.g.,
    /// ```
    /// let query = MyTable
    ///    .Select(name, price.Sum().As("totalPrice"))
    ///    .OrderBy(["totalPrice", "DESC"])
    ///    .Limit(10);
    /// ```
    /// </summary>
    public AggregateFieldExpr<T> As(string alias)
    {
      var copy = new AggregateFieldExpr<T>(this.field, this.op.Op)
      {
        Alias = alias
      };
      return copy;
    }
  }
}