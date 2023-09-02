namespace dpm
{
    /**
     *  A tree of expressions, each of which has an associated name.
     */
    abstract public class FieldExpr
    {
        // A human-readable representation of the expression. Use this to refer to the
        // expression in a `select` or `orderBy`.
        public string name;

        // User-specified alias for expression. Can be used in a `select` and then in
        // a subsequent `orderBy`.
        public string? alias;

        public FieldExpr(string fieldName)
        {
            name = fieldName;
        }

        public override string ToString() {
            return name;
        }

        abstract public Operator Operator();
        abstract public FieldExpr[] Operands();
    }
}
