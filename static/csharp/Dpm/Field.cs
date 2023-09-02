using Google.Protobuf;
using DpmAgent;

namespace Dpm
{
    public class Field<T> : FieldExpr
    {
        public Field(string name) : base(name)
        { }

        public override Operator Operator() { return new Operator.Identity(); }
        public override FieldExpr[] Operands()
        {
            return new[] { this };
        }

        public override IMessage ToDpmProto()
        {
            return new Query.Types.FieldReference()
            {
                FieldName = Name
            };
        }

        public override Query.Types.Expression ToDpmQueryExpression()
        {
            return new Query.Types.Expression()
            {
                Field = new Query.Types.FieldReference()
                {
                    FieldName = Name
                }
            };
        }


        /// <summary>
        /// Alias this field.
        /// E.g.,
        /// <c>
        /// let query = MyTable
        ///    .Select(fieldWithLongName.As('shortName'), price)
        ///    .OrderBy(['shortName', 'DESC'])
        ///    .Limit(10);
        /// </c>
        /// </summary>
        public Field<T> As(string alias)
        {
            var copy = new Field<T>(this.Name)
            {
                Alias = Alias
            };
            return copy;
        }

        private BinaryBooleanFieldExpr AsBooleanExpr(
            BooleanOperatorType op,
            T that
        )
        {
            return new BinaryBooleanFieldExpr(this, op, new LiteralField<T>(that));
        }

        private BinaryBooleanFieldExpr AsBooleanExpr(
            BooleanOperatorType op,
            T[] that
        )
        {
            return new BinaryBooleanFieldExpr(this, op, new LiteralField<T[]>(that));
        }

        private BinaryBooleanFieldExpr AsBooleanExpr(
            BooleanOperatorType op,
            Field<T> that
        )
        {
            return new BinaryBooleanFieldExpr(this, op, that);
        }

        /// <summary>
        /// Returns a 'max' aggregation applied to this field.
        /// </summary>
        public AggregateFieldExpr<T> Max()
        {
            return new AggregateFieldExpr<T>(this, AggregateOperatorType.max);
        }

        /// <summary>
        /// Returns a 'min' aggregation applied to this field.
        /// </summary>
        public AggregateFieldExpr<T> Min()
        {
            return new AggregateFieldExpr<T>(this, AggregateOperatorType.min);
        }

        /// <summary>
        /// Returns a 'count' aggregation applied to this field.
        /// </summary>
        public AggregateFieldExpr<int> Count()
        {
            return new AggregateFieldExpr<int>(this, AggregateOperatorType.count);
        }

        /// <summary>
        /// Returns a distinct 'count' aggregation applied to this field.
        /// </summary>
        public AggregateFieldExpr<int> CountDistinct()
        {
            return new AggregateFieldExpr<int>(this, AggregateOperatorType.countDistinct);
        }

        /// <summary>
        /// Returns an 'average' aggregation applied to this field.
        /// </summary>
        public AggregateFieldExpr<float> Avg()
        {
            return new AggregateFieldExpr<float>(this, AggregateOperatorType.avg);
        }

        /// <summary>
        /// Returns a distinct 'average' aggregation applied to this field.
        /// </summary>
        public AggregateFieldExpr<float> AvgDistinct()
        {
            return new AggregateFieldExpr<float>(this, AggregateOperatorType.avgDistinct);
        }

        /// <summary>
        /// Returns a boolean expression with an equality check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator ==(Field<T> a, T b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.eq, b);
        }

        /// <summary>
        /// Returns a boolean expression with an equality check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator ==(Field<T> a, Field<T> b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.eq, b);
        }

        /// <summary>
        /// Returns a boolean expression with a not equal check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator !=(Field<T> a, T b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.neq, b);
        }

        /// <summary>
        /// Returns a boolean expression with a not equal check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator !=(Field<T> a, Field<T> b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.neq, b);
        }

        /// <summary>
        /// Returns a boolean expression with greater than (>) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator >(Field<T> a, T b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.gt, b);
        }

        /// <summary>
        /// Returns a boolean expression with greater than (>) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator >(Field<T> a, Field<T> b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.gt, b);
        }

        /// <summary>
        /// Returns a boolean expression with greater than or equal (>=) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator >=(Field<T> a, T b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.gte, b);
        }

        /// <summary>
        /// Returns a boolean expression with greater than or equal (>=) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator >=(Field<T> a, Field<T> b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.gte, b);
        }

        /// <summary>
        /// Returns a boolean expression with lesser than (<) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator <(Field<T> a, T b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.lt, b);
        }

        /// <summary>
        /// Returns a boolean expression with lesser than (<) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator <(Field<T> a, Field<T> b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.lt, b);
        }

        /// <summary>
        /// Returns a boolean expression with lesser than or equal (<=) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator <=(Field<T> a, T b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.lte, b);
        }

        /// <summary>
        /// Returns a boolean expression with lesser than or equal (<=) check.
        /// </summary>
        public static BinaryBooleanFieldExpr operator <=(Field<T> a, Field<T> b)
        {
            return a.AsBooleanExpr(BooleanOperatorType.lte, b);
        }

        /// <summary>
        /// Returns a boolean expression with an array membership check. The
        /// field's value must exactly match at least one entry in `that` for
        /// the check to be true.
        /// </summary>
        public BinaryBooleanFieldExpr In(T[] that)
        {
            return this.AsBooleanExpr(BooleanOperatorType.@in, that);
        }

        /// <summary>
        /// Returns a boolean expression that checks if the field's value is in
        /// between a range (inclusive of bounds).
        /// </summary>
        public BinaryBooleanFieldExpr Between(T minVal, T maxVal)
        {
            return (this >= minVal) & (this <= maxVal);
        }

        /// <summary>
        /// Returns a boolean expression that checks if the field is null.
        /// </summary>
        public UnaryBooleanFieldExpr IsNull()
        {
            return new UnaryBooleanFieldExpr(this, UnaryOperatorType.isNull);
        }

        /// <summary>
        /// Returns a boolean expression that checks if the field is not null.
        /// </summary>
        public UnaryBooleanFieldExpr IsNotNull()
        {
            return new UnaryBooleanFieldExpr(this, UnaryOperatorType.isNotNull);
        }

#pragma warning disable CS8765 // Nullability of type of parameter doesn't match overridden member (possibly because of nullability attributes).
        public override bool Equals(object obj)
#pragma warning restore CS8765 // Nullability of type of parameter doesn't match overridden member (possibly because of nullability attributes).
        {
            if (ReferenceEquals(this, obj))
            {
                return true;
            }

            if (obj is null)
            {
                return false;
            }

            throw new NotImplementedException();
        }

        public override int GetHashCode()
        {
            return this.Name.GetHashCode();
        }
    }
}
