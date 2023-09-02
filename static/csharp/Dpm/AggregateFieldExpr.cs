using DpmAgent;
using Google.Protobuf;

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

    private readonly static Dictionary<AggregateOperatorType, Query.Types.AggregateExpression.Types.AggregateOperator> OperatorToPbType = new()
    {
      [AggregateOperatorType.min] = Query.Types.AggregateExpression.Types.AggregateOperator.Min,
      [AggregateOperatorType.max] = Query.Types.AggregateExpression.Types.AggregateOperator.Max,
      [AggregateOperatorType.avg] = Query.Types.AggregateExpression.Types.AggregateOperator.Mean,
      [AggregateOperatorType.avgDistinct] = Query.Types.AggregateExpression.Types.AggregateOperator.Mean,
      [AggregateOperatorType.count] = Query.Types.AggregateExpression.Types.AggregateOperator.Count,
      [AggregateOperatorType.countDistinct] = Query.Types.AggregateExpression.Types.AggregateOperator.CountDistinct,
      [AggregateOperatorType.sum] = Query.Types.AggregateExpression.Types.AggregateOperator.Sum,
    };

    public override IMessage ToDpmProto()
    {
      return new Query.Types.AggregateExpression()
      {
        Argument = field.ToDpmQueryExpression(),
        Op = OperatorToPbType[op.Op]
      };
    }

    public override Query.Types.Expression ToDpmQueryExpression()
    {
      return new Query.Types.Expression()
      {
        Aggregate = (Query.Types.AggregateExpression)ToDpmProto()
      };
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