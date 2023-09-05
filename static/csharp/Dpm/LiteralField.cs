using Google.Protobuf;
using DpmAgent;

namespace Dpm
{
  public class LiteralField<T> : Field<T>
  {
    private readonly T? Value;
    private readonly T[]? Values;

    public LiteralField(T value_) : base($"lit")
    {
      Value = value_;
    }

    public LiteralField(T[] values_) : base($"lit")
    {
      Values = values_;
    }

    public bool IsList()
    {
      return Values != null;
    }

    public T GetValue()
    {
      if (Value == null)
      {
        throw new InvalidDataException("Literal field's value is not set.");
      }
      return Value;
    }

    public T[] GetValues()
    {
      if (Values == null)
      {
        throw new InvalidDataException("Literal field's values are not set.");
      }
      return Values;
    }

    public override IMessage ToDpmProto()
    {
      static Query.Types.Literal makeLit(T s)
      {
        var lit = new Query.Types.Literal();
        if (typeof(T) == typeof(string))
        {
          lit.String = s.ToString();
        }
        else if (typeof(T) == typeof(Int32))
        {
          lit.I32 = Int32.Parse(s.ToString() ?? "0");
        }
        else if (typeof(T) == typeof(Int64))
        {
          lit.I64 = Int64.Parse(s.ToString() ?? "0");
        }
        else if (typeof(T) == typeof(UInt32))
        {
          lit.Ui32 = UInt32.Parse(s.ToString() ?? "0");
        }
        else if (typeof(T) == typeof(UInt64))
        {
          lit.Ui64 = UInt64.Parse(s.ToString() ?? "0");
        }
        else if (typeof(T) == typeof(float))
        {
          lit.F32 = float.Parse(s.ToString() ?? "0");
        }
        else if (typeof(T) == typeof(double))
        {
          lit.F64 = double.Parse(s.ToString() ?? "0");
        }
        else if (typeof(T) == typeof(bool))
        {
          lit.Boolean = bool.Parse(s.ToString() ?? "false");
        }
        else
        {
          lit.Timestamp = DateTime.Parse(s.ToString() ?? "0").Ticks / TimeSpan.TicksPerMillisecond;
        }
        return lit;
      }

      if (IsList())
      {
        var literal = new Query.Types.Literal();
        foreach (var x in Values)
        {
          literal.List.Values.Add(makeLit(x));
        }

        return literal;
      }
      return makeLit(Value);
    }

    public override Query.Types.Expression ToDpmQueryExpression()
    {
      return new Query.Types.Expression()
      {
        Literal = (Query.Types.Literal)ToDpmProto()
      };
    }

    public new AggregateFieldExpr<T> Max()
    {
      throw new NotImplementedException("Cannot call Max on LiteralField");
    }

    public new AggregateFieldExpr<T> Min()
    {
      throw new NotImplementedException("Cannot call Min on LiteralField");
    }

    public new AggregateFieldExpr<T> Sum()
    {
      throw new NotImplementedException("Cannot call Sum on LiteralField");
    }

    public new AggregateFieldExpr<T> Count()
    {
      throw new NotImplementedException("Cannot call Count on LiteralField");
    }

    public new AggregateFieldExpr<T> CountDistinct()
    {
      throw new NotImplementedException("Cannot call CountDistinct on LiteralField");
    }

    public new AggregateFieldExpr<T> Avg()
    {
      throw new NotImplementedException("Cannot call Avg on LiteralField");
    }

    public new AggregateFieldExpr<T> AvgDistinct()
    {
      throw new NotImplementedException("Cannot call AvgDistinct on LiteralField");
    }
  }
}