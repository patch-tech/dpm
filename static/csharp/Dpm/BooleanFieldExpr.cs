namespace Dpm
{

  abstract public class BooleanFieldExpr : FieldExpr {
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
    private readonly Operator op;


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

    /// <summary>
    /// Returns a boolean expression that represents the logical AND: 'a' and 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator &(BinaryBooleanFieldExpr a, UnaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.and, b);
    }

    /// <summary>
    /// Returns a boolean expression that represents the logical AND: 'a' and 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator &(BinaryBooleanFieldExpr a, BinaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.and, b);
    }

    /// <summary>
    /// Returns a boolean expression that represents the logical OR: 'a' or 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator |(BinaryBooleanFieldExpr a, UnaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.or, b);
    }

    /// <summary>
    /// Returns a boolean expression that represents the logical OR: 'a' or 'b'.
    /// </summary>
    public static BinaryBooleanFieldExpr operator |(BinaryBooleanFieldExpr a, BinaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.or, b);
    }
  }
}
