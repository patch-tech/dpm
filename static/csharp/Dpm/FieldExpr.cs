﻿using Google.Protobuf;
using DpmAgent;

namespace Dpm
{
    /// <summary>
    /// A tree of expressions, each of which has an associated name.
    /// </summary>
    abstract public class FieldExpr
    {
        /// A human-readable representation of the expression.
        public string Name;

        /// User-specified alias for expression. Can be used in a `Select` to
        /// alias a selected field.
        public string? Alias;

        public FieldExpr(string fieldName)
        {
            Name = fieldName;
        }

        public override string ToString()
        {
            return Name;
        }

        abstract public Operator Operator();
        abstract public FieldExpr[] Operands();

        /// <summary>
        /// Returns the field expression as a specific DPM proto message defined in dpm_agent.proto
        /// </summary>
        abstract public IMessage ToDpmProto();

        /// <summary>
        /// Returns the field expression as a specific DPM proto message wrapped
        /// in an Expression message as defined in dpm_agent.proto
        /// </summary>
        abstract public Query.Types.Expression ToDpmQueryExpression();
    }
}
