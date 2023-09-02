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
    public readonly string packageId;
    public readonly string datasetName;
    public readonly string datasetVersion;
    public readonly string name;
    private readonly FieldExpr[] fields;
    // private Backend backend?;

    public readonly BooleanFieldExpr? filterExpr;
    public readonly FieldExpr[]? selection;
    public readonly Ordering[]? ordering;
    public readonly uint limitTo;

    private Dictionary<string, FieldExpr> nameToField = new();


    public Table(
      // backend?: Backend;
      string packageId,
      string datasetName,
      string datasetVersion,
      string name,
      FieldExpr[] fields,
      BooleanFieldExpr? filterExpr = null,
      FieldExpr[]? selection = null,
      Ordering[]? ordering = null,
      uint? limitTo = 1_000
    )
    {
      // this.backend = backend;
      this.packageId = packageId;
      this.datasetName = datasetName;
      this.datasetVersion = datasetVersion;
      this.name = name;
      this.fields = (FieldExpr[])fields.Clone();
      this.filterExpr = filterExpr;
      if (selection != null)
      {
        this.selection = (FieldExpr[])selection.Clone();
      }
      if (ordering != null)
      {
        this.ordering = (Ordering[])ordering.Clone();
      }

      if (limitTo != null && limitTo > 0)
      {
        this.limitTo = (uint)limitTo;
      }

      Dictionary<string, FieldExpr> emptyMap = new();
      this.nameToField = this.fields.Aggregate(emptyMap, (acc, field) =>
      {
        acc[field.Name] = field;
        return acc;
      });

      // this.getOrMakeBackend();
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
        packageId: packageId,
        datasetName: datasetName,
        datasetVersion: datasetVersion,
        name: name ?? this.name,
        fields: fields ?? this.fields,
        filterExpr: filterExpr ?? this.filterExpr,
        selection: selection ?? this.selection,
        ordering: ordering ?? this.ordering,
        limitTo: limitTo ?? this.limitTo
        );
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
          return Array.Find(selection ?? Array.Empty<FieldExpr>(), s => s.Alias == name);
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
    ///   MyTable["CATEGORY"],
    ///   saleDate.month.As("saleMonth"),
    ///   price.Avg().As("meanPrice")
    /// )
    /// .OrderBy((MyTable["meanPrice"], "DESC'), (saleDate.Month, "ASC"))
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

    // TODO(ajith): Complete remaining methods, compile and execute.
  }
}
