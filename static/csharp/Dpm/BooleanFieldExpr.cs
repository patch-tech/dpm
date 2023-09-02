namespace dpm
{
  /**
   * A binary boolean expression. Can be combined with other boolean expressions
   * using `and`, `or` methods.
   */
  public class BooleanFieldExpr : FieldExpr
  {
    private FieldExpr field;
    private BooleanOperatorType opType;
    private FieldExpr other;
    private Operator op;


    public BooleanFieldExpr(
    FieldExpr field_,
    BooleanOperatorType opType_,
    FieldExpr other_
  ) : base($"({field_.name} ${opType_} ${other_.name})")
    {
      field = field_;
      opType = opType_;
      other = other_;
      op = new Operator(opType_.ToString());
    }

    public override Operator Operator()
    {
      return op;
    }

    public override FieldExpr[] Operands()
    {
      return new[] { this.field, this.other };
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
