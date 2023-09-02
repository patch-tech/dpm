namespace dpm
{

  public enum Direction
  {
    ASC,
    DESC
  }

  public class Ordering
  {
    public readonly FieldExpr field;
    public readonly Direction direction;

    public Ordering(FieldExpr field_, Direction dir_)
    {
      this.field = field_;
      this.direction = dir_;
    }
  }

  /**
  * The entry point to query building. The flow:
  *   1. Starting with an instance of `Table`, `select` columns.
  *   2. Optionally, `filter`, `groupBy`, `orderBy`, `limit`.
  *   3. Compile and/or execute the formulated query against an execution backend.
  * N.B.: Avoid a direct instantiation of `Table`! select from one of the
  * generated table classes to obtain a Table.
  */
  public class Table
  {
    public readonly string packageId;
    public readonly string datasetName;
    public readonly string datasetVersion;
    public readonly string name;
    private FieldExpr[] fields;
    // private Backend backend?;

    public readonly BooleanFieldExpr? filterExpr;
    public readonly FieldExpr[]? selection;
    public readonly Ordering[]? ordering;
    public readonly decimal limitTo;

    private Dictionary<string, FieldExpr> nameToField =
      new Dictionary<string, FieldExpr>();


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
      decimal? limitTo = 1_000
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
        this.limitTo = (decimal)limitTo;
      }

      Dictionary<string, FieldExpr> emptyMap = new Dictionary<string, FieldExpr>();
      this.nameToField = this.fields.Aggregate(emptyMap, (acc, field) =>
      {
        acc[field.name] = field;
        return acc;
      });

      // this.getOrMakeBackend();
    }

    public Table Select(params FieldExpr[] selection) {
      // TODO(ajith): Create a copy of this table with its selection field set to the input.
      return this;
    }

    // TODO(ajith): Complete remaining methods.
  }
}
