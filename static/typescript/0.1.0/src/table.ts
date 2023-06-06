import { makeBackend } from './backends/factory';
import { Backend } from './backends/interface';
import { Dataset } from './dataset';
import { BooleanFieldExpr, FieldExpr } from './field_expr';

export type Ordering = [FieldExpr, 'ASC' | 'DESC'];
export type Selector = string | FieldExpr;

/**
 * The entry point to query building. The flow:
 *   1. Starting with an instance of `Table`, `select` columns.
 *   2. Optionally, `filter`, `groupBy`, `orderBy`, `limit`.
 *   3. Compile and/or execute the formulated query against an execution backend.
 * N.B.: Avoid a direct instantiation of `Table`! select from one of the
 * generated table classes to obtain a Table.
 */
export class Table {
  readonly dataset: Dataset;
  readonly source?: string;
  readonly name: string;
  private fields: FieldExpr[];
  private backend?: Backend;

  readonly filterExpr?: BooleanFieldExpr;
  readonly selection?: FieldExpr[];
  readonly ordering?: Ordering[];
  readonly limitTo: number;

  private nameToField: { [key: string]: FieldExpr } = {};

  constructor({
    backend,
    dataset,
    source,
    name,
    fields,
    filterExpr,
    selection,
    ordering,
    limitTo = 1_000,
  }: {
    backend?: Backend;
    dataset: Dataset;
    source?: string;
    name: string;
    fields: FieldExpr[];
    filterExpr?: BooleanFieldExpr;
    selection?: FieldExpr[];
    ordering?: Ordering[];
    limitTo?: number;
  }) {
    this.backend = backend;
    this.dataset = dataset;
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
  }

  private copy(args: {
    name?: string;
    fields?: FieldExpr[];
    filterExpr?: BooleanFieldExpr;
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

  private getOrMakeBackend(): Backend | undefined {
    if (this.backend === undefined) {
      this.backend = makeBackend(this);
    }

    return this.backend;
  }

  filter(expr: BooleanFieldExpr): Table {
    return this.copy({ filterExpr: expr });
  }

  select(...selection: Selector[]): Table {
    let selectExprs: FieldExpr[] = selection.map((s) => {
      return this.selectedFieldExpr(s);
    });
    return this.copy({ selection: selectExprs });
  }

  orderBy(...ordering: [Selector, 'ASC' | 'DESC'][]): Table {
    let orderingExpr: Ordering[] = ordering.map(([sel, dir]) => {
      return [this.selectedFieldExpr(sel), dir];
    });
    return this.copy({ ordering: orderingExpr });
  }

  // Returns a new `Table`, otherwise the same but limited to the given `n` many rows.
  limit(n: number): Table {
    return this.copy({ limitTo: n });
  }

  compile(): Promise<string> {
    const backend = this.getOrMakeBackend();
    if (backend) {
      return backend.compile(this);
    }

    return Promise.reject(
      'Failed to find a suitable backend to compile this query'
    );
  }

  execute<Row>(): Promise<Row[]> {
    const backend = this.getOrMakeBackend();
    if (backend) {
      return backend.execute(this);
    }

    return Promise.reject(
      'Failed to find a suitable backend to execute this query'
    );
  }
}
