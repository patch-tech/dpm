using System.Security.Cryptography.X509Certificates;
using DpmAgent;

namespace Dpm
{

  /// Builds a DpmAgent Query from a table expression.
  public static class DpmAgentQueryFactory
  {
    static Query.Types.SelectExpression MakeSelectExpression(FieldExpr expr)
    {
      var selectExpr = new Query.Types.SelectExpression
      {
        Argument = expr.ToDpmQueryExpression()
      };
      if (expr.Alias != null)
      {
        selectExpr.Alias = expr.Alias;
      }
      return selectExpr;
    }

    static Query.Types.GroupByExpression MakeGroupByExpression(FieldExpr expr)
    {
      var dpmGroupBy = new Query.Types.GroupByExpression();
      if (expr.GetType().Name == typeof(DerivedField<,>).Name)
      {
        dpmGroupBy.Derived = (Query.Types.DerivedExpression)expr.ToDpmProto();
      }
      else if (expr.Operator().GetType().Name == typeof(Operator.Identity).Name)
      {
        dpmGroupBy.Field = (Query.Types.FieldReference)expr.ToDpmProto();
      }
      else
      {
        throw new InvalidDataException($"Unexpected field expression in groupBy: {expr}");
      }

      return dpmGroupBy;
    }

    static Query.Types.OrderByExpression MakeOrderByExpression(Ordering ordering)
    {
      var (fieldExpr, direction) = ordering;
      return new Query.Types.OrderByExpression()
      {
        Argument = fieldExpr.ToDpmQueryExpression(),
        Direction = direction == Direction.ASC ? Query.Types.OrderByExpression.Types.Direction.Asc : Query.Types.OrderByExpression.Types.Direction.Desc
      };
    }

    public static Query MakeQuery(Table query)
    {
      var dpmQuery = new Query
      {
        Id = new Query.Types.Id
        {
          PackageId = query.PackageId
        },
        ClientVersion = new ClientVersion
        {
          DatasetVersion = query.DatasetVersion,
          CodeVersion = Constants.CODE_VERSION,
          Client = ClientVersion.Types.Client.Csharp
        },
        SelectFrom = query.Name
      };

      if (query.Selection != null)
      {
        foreach (var item in query.Selection)
        {
          dpmQuery.Select.Add(MakeSelectExpression(item));
        }

        // Process any groupings defined in selection.
        if (query.Selection.Where((x) => x.GetType().Name == typeof(AggregateFieldExpr<>).Name).Any())
        {
          // Group by the non-aggregate selections.
          var grouping = query.Selection.Where((x) => x.GetType().Name != typeof(AggregateFieldExpr<>).Name);
          foreach (var item in grouping)
          {
            dpmQuery.GroupBy.Add(MakeGroupByExpression(item));
          }
        }
      }

      // Process filter.
      if (query.FilterExpr != null)
      {
        dpmQuery.Filter = (Query.Types.BooleanExpression)query.FilterExpr.ToDpmProto();
      }

      // Process orderBy.
      if (query.Ordering != null)
      {
        foreach (var item in query.Ordering)
        {
          dpmQuery.OrderBy.Add(MakeOrderByExpression(item));
        }
      }

      if (query.LimitTo > 0)
      {
        dpmQuery.Limit = query.LimitTo;
      }

      return dpmQuery;
    }
  }

}