namespace dpm
{
  /**
   * A unary boolean expression.
   * E.g., a null check on a field can be expressed using a UnaryBooleanFieldExpr:
   * ```
   * var nameField = new Field<string>("name");
   * const isNameNotNull = new UnaryBooleanFieldExpr(nameField, UnaryOperatorType.isNotNull);
   * ```
   */
  public class UnaryBooleanFieldExpr : FieldExpr
  {
    private FieldExpr field;
    private UnaryOperatorType opType;
    private Operator op;


    public UnaryBooleanFieldExpr(
    FieldExpr field_,
    UnaryOperatorType opType_) : base($"(${opType_}({field_.name}))")
    {
      field = field_;
      opType = opType_;
      op = new Operator(opType_.ToString());
    }

    public override Operator Operator()
    {
      return op;
    }

    public override FieldExpr[] Operands()
    {
      return new[] { this.field };
    }

    public BooleanFieldExpr And(BooleanFieldExpr that)
    {
      return new BooleanFieldExpr(this, BooleanOperatorType.and, that);
    }

    public BooleanFieldExpr And(UnaryBooleanFieldExpr that)
    {
      return new BooleanFieldExpr(this, BooleanOperatorType.and, that);
    }

    public BooleanFieldExpr Or(BooleanFieldExpr that)
    {
      return new BooleanFieldExpr(this, BooleanOperatorType.or, that);
    }

    public BooleanFieldExpr Or(UnaryBooleanFieldExpr that)
    {
      return new BooleanFieldExpr(this, BooleanOperatorType.or, that);
    }
  }
}
