import { makeBackend } from './backends/factory';
import { Backend } from './backends/interface';
import { BooleanFieldExpr, FieldExpr, UnaryBooleanFieldExpr } from './field_expr';

export type Ordering = [FieldExpr, 'ASC' | 'DESC'];
export type Selector = string | FieldExpr;

/**
 * The entry point to query building. The flow:
 *   1. Starting with an instance of `Table`, `select` columns.
 *   2. Optionally, `filter`, `orderBy`, `limit`.
 *   3. Compile and/or execute the formulated query against an execution backend.
 * N.B.: Avoid a direct instantiation of `Table`! select from one of the
 * generated table classes to obtain a Table.
 */
export class Table {
  readonly packageId: string;
  readonly datasetName: string;
  readonly datasetVersion: string;
  readonly source?: string;
  readonly name: string;
  private fields: FieldExpr[];
  private backend?: Backend;

  readonly filterExpr?: BooleanFieldExpr | UnaryBooleanFieldExpr;
  readonly selection?: FieldExpr[];
  readonly ordering?: Ordering[];
  readonly limitTo: number;

  private nameToField: { [key: string]: FieldExpr } = {};

  constructor({
    backend,
    packageId,
    datasetName,
    datasetVersion,
    source,
    name,
    fields,
    filterExpr,
    selection,
    ordering,
    limitTo = 1_000,
  }: {
    backend?: Backend;
    packageId: string;
    datasetName: string;
    datasetVersion: string;
    source?: string;
    name: string;
    fields: FieldExpr[];
    filterExpr?: BooleanFieldExpr | UnaryBooleanFieldExpr;
    selection?: FieldExpr[];
    ordering?: Ordering[];
    limitTo?: number;
  }) {
    this.backend = backend;
    this.packageId = packageId;
    this.datasetName = datasetName;
    this.datasetVersion = datasetVersion;
    this.source = source;
    this.name = name;
    this.fields = [...fields];
    this.filterExpr = filterExpr;
    if (selection) {
      this.selection = [...selection];
    }
    if (ordering) {
      this.ordering = [...ordering];
    }
    this.limitTo = limitTo;

    let emptyMap: { [key: string]: FieldExpr } = {};
    this.nameToField = this.fields.reduce((acc, field) => {
      acc[field.name] = field;
      return acc;
    }, emptyMap);

    this.getOrMakeBackend();
  }

  private copy(args: {
    name?: string;
    fields?: FieldExpr[];
    filterExpr?: BooleanFieldExpr | UnaryBooleanFieldExpr;
    selection?: FieldExpr[];
    ordering?: Ordering[];
    limitTo?: number;
  }): Table {
    return new Table({ ...this, ...args });
  }

  private selectedFieldExpr(selector: Selector): FieldExpr {
    if (selector instanceof FieldExpr) {
      return selector;
    } else if (selector in this.nameToField) {
      return this.nameToField[selector];
    } else {
      throw new Error(`Unknown field selector ${selector}`);
    }
  }

  private findSelectionByAlias(alias: string): FieldExpr | undefined {
    return this.selection?.find((f) => f.alias === alias);
  }

  private getOrMakeBackend(): Backend | undefined {
    if (this.backend === undefined) {
      this.backend = makeBackend();
    }

    return this.backend;
  }

  /**
   * Sets the filter expression for the table.
   * @param expr Boolean expression to filter by.
   * @returns Copy of table with filter set.
   */
  filter(expr: BooleanFieldExpr | UnaryBooleanFieldExpr): Table {
    return this.copy({ filterExpr: expr });
  }

  /**
   * Sets the fields to select from the table. Accepts a mix of field
   * expressions, and field name strings. One may specify a mix of fields,
   * derived fields, and aggregate field expressions.
   * E.g.,
   * ```
   * let query = MyTable.select(
   *   name,
   *   'CATEGORY',
   *   saleDate.month.as('saleMonth'),
   *   price.avg().as('meanPrice')
   * ).limit(10);
   * ```
   * @param selection Fields to select. Accepts both field expressions, or field name strings.
   * @returns Copy of table with field selection set.
   */
  select(...selection: Selector[]): Table {
    let selectExprs: FieldExpr[] = selection.map((s) => {
      return this.selectedFieldExpr(s);
    });
    return this.copy({ selection: selectExprs });
  }

  /**
   * Set the tables ordering columns with their sort direction. The column
   * selectors can be field expressions or strings that refer to table fields
   * aliases of selected fields.
   * E.g.,
   * ```
   * let query = MyTable.select(
   *   name,
   *   'CATEGORY',
   *   saleDate.month.as('saleMonth'),
   *   price.avg().as('meanPrice')
   * ).orderBy(['meanPrice', 'DESC'], [saleDate.month, 'ASC'])
   * .limit(10);
   * ```
   * @param ordering (selector, direction) pairs.
   * @returns Copy of table with ordering set.
   */
  orderBy(...ordering: [Selector, 'ASC' | 'DESC'][]): Table {
    let orderingExpr: Ordering[] = ordering.map(([sel, dir]) => {
      try {
        return [this.selectedFieldExpr(sel), dir];
      } catch (e) {
        // The selector might be an alias of a selected field.
        if (typeof sel === 'string') {
          const fieldExpr = this.findSelectionByAlias(sel);
          if (fieldExpr !== undefined) {
            return [fieldExpr, dir];
          }
        }
        // No field expression found, re-throw error.
        throw new Error(`Unknown field selector ${sel}`);
      }
    });
    return this.copy({ ordering: orderingExpr });
  }

  /**
   * Sets the row limit on the table.
   * @param n limit value.
   * @returns Copy of table with limit set to 'n'.
   */
  limit(n: number): Table {
    return this.copy({ limitTo: n });
  }

  /**
   * Compiles the table expression into a query string on its execution backend.
   * E.g., returns a Snowsql string for a table expression with a Snowflake
   * execution backend.
   * @returns Compiled query string.
   */
  compile(): Promise<string> {
    const backend = this.getOrMakeBackend();
    if (backend) {
      return backend.compile(this);
    }

    return Promise.reject(
      'Failed to find a suitable backend to compile this query'
    );
  }

  /**
   * Executes the table expression on its execution backend and returns a
   * promise that resolves to the results.
   * @returns Result of executing the table expression on its execution backend.
   */
  execute<Row extends object>(): Promise<Row[]> {
    const backend = this.getOrMakeBackend();
    if (backend) {
      return backend.execute(this);
    }

    return Promise.reject(
      'Failed to find a suitable backend to execute this query'
    );
  }
}
