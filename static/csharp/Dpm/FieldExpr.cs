namespace Dpm
{
    /// <summary>
    /// A tree of expressions, each of which has an associated name.
    /// </summary>
    abstract public class FieldExpr
    {
        /// A human-readable representation of the expression. Use this to refer to the
        /// expression in a `select` or `orderBy`.
        public string Name;

        /// User-specified alias for expression. Can be used in a `select` and then in
        /// a subsequent `orderBy`.
        public string? Alias;

        public FieldExpr(string fieldName)
        {
            Name = fieldName;
        }

        public override string ToString() {
            return Name;
        }

        abstract public Operator Operator();
        abstract public FieldExpr[] Operands();
    }
}
