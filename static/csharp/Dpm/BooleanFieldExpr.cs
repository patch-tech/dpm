using DpmAgent;
using Google.Protobuf;

namespace Dpm
{

  abstract public class BooleanFieldExpr : FieldExpr
  {
    public BooleanFieldExpr(string fieldName) : base(fieldName) { }
  }

  /// <summary>
  /// A binary boolean expression. Can be combined with other boolean expressions
  /// using `and`, `or` methods.
  /// </summary>
  public class BinaryBooleanFieldExpr : BooleanFieldExpr
  {
    private readonly FieldExpr field;
    private readonly FieldExpr other;
    private readonly Operator.Boolean op;

    private readonly static Dictionary<BooleanOperatorType, Query.Types.BooleanExpression.Types.BooleanOperator> OperatorToPbType = new()
    {
      [BooleanOperatorType.and] = Query.Types.BooleanExpression.Types.BooleanOperator.And,
      [BooleanOperatorType.or] = Query.Types.BooleanExpression.Types.BooleanOperator.Or,
      [BooleanOperatorType.eq] = Query.Types.BooleanExpression.Types.BooleanOperator.Eq,
      [BooleanOperatorType.neq] = Query.Types.BooleanExpression.Types.BooleanOperator.Neq,
      [BooleanOperatorType.gt] = Query.Types.BooleanExpression.Types.BooleanOperator.Gt,
      [BooleanOperatorType.gte] = Query.Types.BooleanExpression.Types.BooleanOperator.Gte,
      [BooleanOperatorType.lt] = Query.Types.BooleanExpression.Types.BooleanOperator.Lt,
      [BooleanOperatorType.lte] = Query.Types.BooleanExpression.Types.BooleanOperator.Lte,
      [BooleanOperatorType.like] = Query.Types.BooleanExpression.Types.BooleanOperator.Like,
      [BooleanOperatorType.@in] = Query.Types.BooleanExpression.Types.BooleanOperator.In,
    };

    public BinaryBooleanFieldExpr(
    FieldExpr field_,
    BooleanOperatorType opType_,
    FieldExpr other_
  ) : base($"({field_.Name} ${opType_} ${other_.Name})")
    {
      field = field_;
      other = other_;
      op = new Operator.Boolean(opType_);
    }

    public override Operator Operator()
    {
      return op;
    }

    public override FieldExpr[] Operands()
    {
      return new[] { this.field, this.other };
    }

    public override IMessage ToDpmProto()
    {
      var booleanExpr = new Query.Types.BooleanExpression()
      {
        Op = OperatorToPbType[op.Op]
      };

      foreach (var x in Operands())
      {
        booleanExpr.Arguments.Add(x.ToDpmQueryExpression());
      }

      return booleanExpr;
    }

    public override Query.Types.Expression ToDpmQueryExpression()
    {
      return new Query.Types.Expression()
      {
        Condition = (Query.Types.BooleanExpression)ToDpmProto()
      };
    }

    /// <summary>
    /// Returns a boolean expression that represents the logical AND: 'a' and 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator &(BinaryBooleanFieldExpr a, UnaryBooleanFieldExpr b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.and, b);
    }

    /// <summary>
    /// Returns a boolean expression that represents the logical AND: 'a' and 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator &(BinaryBooleanFieldExpr a, BinaryBooleanFieldExpr b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.and, b);
    }

    /// <summary>
    /// Returns a boolean expression that represents the logical OR: 'a' or 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator |(BinaryBooleanFieldExpr a, UnaryBooleanFieldExpr b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.or, b);
    }

    /// <summary>
    /// Returns a boolean expression that represents the logical OR: 'a' or 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator |(BinaryBooleanFieldExpr a, BinaryBooleanFieldExpr b)
    {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.or, b);
    }
  }
}
