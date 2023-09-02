namespace dpm
{
    public class Field<T> : FieldExpr
    {
        public Field(string name) : base(name)
        { }

        public override Operator Operator() { return new Operator(dpm.Operator.ident); }
        public override FieldExpr[] Operands()
        {
            return new[] { this };
        }

        // TODO(ajith): Add methods.

    }

    // TODO(ajith): Move these fields to their own file.
    public class LiteralField<T> : Field<T>
    {
        public Scalar<T> value;

        public LiteralField(T value_) : base($"lit(${value_})")
        {
            value = new Scalar<T>(value_);
        }

        public LiteralField(T[] value_) : base($"lit(${value_})")
        {
            value = new Scalar<T>(value_);
        }
    }

    public class StringField : Field<string>
    {
        public StringField(string name) : base(name) { }

        /**
          * Returns a boolean expression for a string `like` check.
          * See: https://en.wikibooks.org/wiki/Structured_Query_Language/Like_Predicate#LIKE
          * E.g.,
          * ```
          * var query = MyTable
          *    .select(name, price)
          *    .filter(name.Like('%shirt%'))
          *    .limit(10);
          * ```
          * @param pattern The like pattern with wildcards: % (one or more) and _ (exactly one).
          * @returns
          */
        public BooleanFieldExpr Like(string pattern)
        {
            return new BooleanFieldExpr(this, BooleanOperatorType.like, new LiteralField<string>(pattern));
        }
    }

    public class DateField : Field<System.DateOnly>
    {
        public DateField(string name) : base(name)
        {
        }

        // TODO(ajith): define date methods/properties.
    }

    public class DateTimeField : Field<System.DateTime>
    {
        public DateTimeField(string name) : base(name)
        {
        }

        // TODO(ajith): define date methods/properties.
    }
}
