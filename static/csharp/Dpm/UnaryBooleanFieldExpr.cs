namespace Dpm
{
  /// <summary>
  /// A unary boolean expression.
  /// E.g., a null check on a field can be expressed using a UnaryBooleanFieldExpr:
  /// <c>
  /// var nameField = new Field<string>("name");
  /// const isNameNotNull = new UnaryBooleanFieldExpr(nameField, UnaryOperatorType.isNotNull);
  /// </c>
  /// </summary>
  public class UnaryBooleanFieldExpr : BooleanFieldExpr
  {
    private readonly FieldExpr field;
    private readonly Operator op;


    public UnaryBooleanFieldExpr(
    FieldExpr field_,
    UnaryOperatorType opType_) : base($"(${opType_}({field_.Name}))")
    {
      field = field_;
      op = new Operator.Unary(opType_);
    }

    public override Operator Operator()
    {
      return op;
    }

    public override FieldExpr[] Operands()
    {
      return new[] { this.field };
    }

    public static BinaryBooleanFieldExpr operator &(UnaryBooleanFieldExpr a, BinaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.and, b);
    }

    public static BinaryBooleanFieldExpr operator &(UnaryBooleanFieldExpr a, UnaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.and, b);
    }

    public static BinaryBooleanFieldExpr operator |(UnaryBooleanFieldExpr a, BinaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.or, b);
    }

    public static BinaryBooleanFieldExpr operator |(UnaryBooleanFieldExpr a, UnaryBooleanFieldExpr b) {
      return new BinaryBooleanFieldExpr(a, BooleanOperatorType.or, b);
    }
  }
}
