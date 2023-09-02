namespace Dpm
{
  /// <summary>
  /// A derived field obtained by applying a projection operator.
  /// E.g.
  /// <c>
  /// var startDateTime = new DateTimeField("started_at");
  /// var startYear = new DerivedField<int, Date>(startDateTime, ProjectionOperatorType.year);
  /// </c>
  ///
  /// For getters that return derived fields:
  /// <see cref="DateField.Year"/>
  /// <see cref="DateField.Month"/>
  /// <see cref="DateField.Day"/>
  /// </summary>
  public class DerivedField<T, U> : Field<T>
  {
    private readonly Operator.Projection op;
    private readonly Field<U> field;

    public DerivedField(Field<U> field, ProjectionOperatorType opType) : base($"(${opType}(${field.Name}))")
    {
      this.field = field;
      this.op = new Operator.Projection(opType);
    }

    public override Operator Operator()
    {
      return this.op;
    }

    public override FieldExpr[] Operands()
    {
      return new[] { this.field };
    }

    public new DerivedField<T, U> As(string alias)
    {
      var copy = new DerivedField<T, U>(this.field, this.op.Op)
      {
        Alias = alias
      };
      return copy;
    }
  }
}