using Newtonsoft.Json;

namespace Dpm
{

  public enum Direction
  {
    ASC,
    DESC
  }

  public record Ordering(FieldExpr Field, Direction Dir);

  /// <summary>
  ///  The entry point to query building. The flow:
  ///    1. Starting with an instance of `Table`, `Select` columns.
  ///    2. Optionally, `Filter`, `OrderBy`, `Limit`.
  ///    3. Compile and/or execute the formulated query against an execution backend.
  ///  N.B.: Avoid a direct instantiation of `Table`! select from one of the
  ///  generated table classes to obtain a Table.
  /// </summary>
  public class Table
  {
    public readonly string PackageId;
    public readonly string DatasetName;
    public readonly string DatasetVersion;
    public readonly string Name;
    private readonly FieldExpr[] Fields;
    private DpmAgentClient? Backend;

    public readonly BooleanFieldExpr? FilterExpr;
    public readonly FieldExpr[]? Selection;
    public readonly Ordering[]? Ordering;
    public readonly uint LimitTo;

    private readonly Dictionary<string, FieldExpr> nameToField = new();


    public Table(
      string packageId,
      string datasetName,
      string datasetVersion,
      string name,
      FieldExpr[] fields,
      BooleanFieldExpr? filterExpr = null,
      FieldExpr[]? selection = null,
      Ordering[]? ordering = null,
      uint? limitTo = 1_000,
      DpmAgentClient? backend = null
    )
    {
      this.Backend = backend;
      this.PackageId = packageId;
      this.DatasetName = datasetName;
      this.DatasetVersion = datasetVersion;
      this.Name = name;
      this.Fields = (FieldExpr[])fields.Clone();
      this.FilterExpr = filterExpr;
      if (selection != null)
      {
        this.Selection = (FieldExpr[])selection.Clone();
      }
      if (ordering != null)
      {
        this.Ordering = (Ordering[])ordering.Clone();
      }

      if (limitTo != null && limitTo > 0)
      {
        this.LimitTo = (uint)limitTo;
      }

      Dictionary<string, FieldExpr> emptyMap = new();
      this.nameToField = this.Fields.Aggregate(emptyMap, (acc, field) =>
      {
        acc[field.Name] = field;
        return acc;
      });

      GetOrMakeBackend();
    }

    private Table Copy(
      string? name = null,
      FieldExpr[]? fields = null,
      BooleanFieldExpr? filterExpr = null,
      FieldExpr[]? selection = null,
      Ordering[]? ordering = null,
      uint? limitTo = null)
    {
      return new Table(
        packageId: PackageId,
        datasetName: DatasetName,
        datasetVersion: DatasetVersion,
        name: name ?? Name,
        fields: fields ?? Fields,
        filterExpr: filterExpr ?? FilterExpr,
        selection: selection ?? Selection,
        ordering: ordering ?? Ordering,
        limitTo: limitTo ?? LimitTo,
        backend: Backend
        );
    }

    private DpmAgentClient GetOrMakeBackend()
    {
      if (Backend == null)
      {
        Backend = DpmAgentClientFactory.MakeClient();
      }
      return Backend;
    }

    /// <summary>
    /// Indexer. Returns FieldExpr by searching in available fields by name, or selections by alias.
    /// </summary>
    public FieldExpr? this[string name]
    {
      get
      {
        if (nameToField.ContainsKey(name)) { return nameToField[name]; }
        else
        {
          return Array.Find(Selection ?? Array.Empty<FieldExpr>(), s => s.Alias == name);
        }
      }
    }

    /// <summary>
    /// Sets the filter expression for the table.
    ///  E.g.,
    /// <c>
    /// var query = MyTable.Select(
    ///   name,
    ///   category,
    ///   saleDate.Month.As("saleMonth"),
    /// )
    /// .Filter(
    ///   category.In(new string[]{"shirts", "tops"}) & saleDate.Month < 5
    /// ).Limit(10);
    /// </c>
    /// </summary>
    public Table Filter(BooleanFieldExpr expr)
    {
      return Copy(filterExpr: expr);
    }

    /// <summary>
    /// Sets the fields to select from the table.  One may specify a mix of
    /// fields, derived fields, and aggregate field expressions.
    /// E.g.,
    /// <c>
    /// var query = MyTable.Select(
    ///   name,
    ///   category,
    ///   saleDate.Month.As("saleMonth"),
    ///   price.Avg().As("meanPrice")
    /// ).Limit(10);
    /// </c>
    /// <summary>
    /// <returns> Copy of table with field selection set.</returns>
    public Table Select(params FieldExpr[] selection)
    {
      return Copy(selection: selection);
    }


    /// <summary>
    /// Set the tables ordering columns with their sort direction.
    /// E.g.,
    /// <c>
    /// var query = MyTable.select(
    ///   name,
    ///   category,
    ///   saleDate.Month.As("saleMonth"),
    ///   price.Avg().As("meanPrice")
    /// )
    /// .OrderBy((price.Avg(), "DESC'), (saleDate.Month, "ASC"))
    /// .Limit(10);
    /// </c>
    /// </summary>
    public Table OrderBy(params (FieldExpr Field, Direction Dir)[] ordering)
    {
      return Copy(ordering: ordering.Select(x => new Ordering(x.Field, x.Dir)).ToArray());
    }

    /// <summary>
    /// Sets the row limit on the table.
    /// </summary>
    public Table Limit(uint n)
    {
      return Copy(limitTo: n);
    }

    /// <summary>
    /// Compiles the table expression into a query string on its execution backend.
    /// E.g., returns a Snowsql string for a table expression with a Snowflake
    /// execution backend.
    /// </summary>
    public string Compile()
    {
      var backend = GetOrMakeBackend();
      var dpmQuery = DpmAgentQueryFactory.MakeQuery(this);
      return backend.CompileQuery(dpmQuery);
    }

    /// <summary>
    /// Returns the results from executing the query.
    /// </summary>
    /// <typeparam name="T">The type of the expected result entries</typeparam>
    /// <returns>An array of values of type T</returns>
    public async Task<T[]> Execute<T>()
    {
      var backend = GetOrMakeBackend();
      var dpmQuery = DpmAgentQueryFactory.MakeQuery(this);
      var resultTask = backend.ExecuteQueryAsync(dpmQuery);
      if (resultTask == null)
      {
        Console.Error.WriteLine("Got null result executing query");
        return Array.Empty<T>();
      }

      var result = await resultTask;

      try
      {
        return JsonConvert.DeserializeObject<T[]>(result.JsonData) ?? Array.Empty<T>();
      }
      catch (Exception e)
      {
        Console.Error.WriteLine("Error when JSON deserializing query results", e);
      }
      return Array.Empty<T>();
    }

    /// <summary>
    /// Returns a dynamic instance of the results from executing the query.
    /// The return type of a successful deserialization should be Newtonsoft.Json.Linq.JArray
    /// The caller can then iterate on the results and directly access each entry's fields.
    /// <c>
    /// var results = await MyTable.Select(name.As("Name")).Limit(10).Execute();
    /// Console.WriteLine($"Got {results.Count} result entries");
    /// foreach(var item in results) {
    ///   Console.WriteLine($"Name = {item.Name}");
    /// }
    /// </c>
    /// </summary>
    /// <returns></returns>
    public async Task<dynamic> Execute()
    {
      var backend = GetOrMakeBackend();
      var dpmQuery = DpmAgentQueryFactory.MakeQuery(this);
      var resultTask = backend.ExecuteQueryAsync(dpmQuery);
      if (resultTask == null)
      {
        Console.Error.WriteLine("Got null result executing query");
        return Array.Empty<Object>();
      }

      var result = await resultTask;
      try
      {
        return JsonConvert.DeserializeObject(result.JsonData) ?? Array.Empty<object>();
      }
      catch (Exception e)
      {
        Console.Error.WriteLine("Error when JSON deserializing query results", e);
      }
      return Array.Empty<object>();
    }
  }
}
