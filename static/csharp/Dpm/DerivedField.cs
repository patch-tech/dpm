using DpmAgent;
using Google.Protobuf;

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

    private readonly static Dictionary<ProjectionOperatorType, Query.Types.DerivedExpression.Types.ProjectionOperator> OperatorToPbType = new()
    {
      [ProjectionOperatorType.day] = Query.Types.DerivedExpression.Types.ProjectionOperator.Day,
      [ProjectionOperatorType.dayOfWeek] = Query.Types.DerivedExpression.Types.ProjectionOperator.DayOfWeek,
      [ProjectionOperatorType.week] = Query.Types.DerivedExpression.Types.ProjectionOperator.Week,
      [ProjectionOperatorType.month] = Query.Types.DerivedExpression.Types.ProjectionOperator.Month,
      [ProjectionOperatorType.year] = Query.Types.DerivedExpression.Types.ProjectionOperator.Year,
      [ProjectionOperatorType.date] = Query.Types.DerivedExpression.Types.ProjectionOperator.Date,
      [ProjectionOperatorType.time] = Query.Types.DerivedExpression.Types.ProjectionOperator.Time,
      [ProjectionOperatorType.hour] = Query.Types.DerivedExpression.Types.ProjectionOperator.Hour,
      [ProjectionOperatorType.minute] = Query.Types.DerivedExpression.Types.ProjectionOperator.Minute,
      [ProjectionOperatorType.second] = Query.Types.DerivedExpression.Types.ProjectionOperator.Second,
      [ProjectionOperatorType.millisecond] = Query.Types.DerivedExpression.Types.ProjectionOperator.Millisecond,
    };

    public override IMessage ToDpmProto()
    {
      return new Query.Types.DerivedExpression()
      {
        Argument = field.ToDpmQueryExpression(),
        Op = OperatorToPbType[op.Op]
      };
    }

    public override Query.Types.Expression ToDpmQueryExpression()
    {
      return new Query.Types.Expression()
      {
        Derived = (Query.Types.DerivedExpression)ToDpmProto()
      };
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