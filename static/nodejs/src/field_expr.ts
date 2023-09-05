export type Scalar =
  | string
  | number
  | boolean
  | Date;

export type UnaryOperator = 'isNull' | 'isNotNull';
export type BooleanOperator =
  | 'eq'
  | 'neq'
  | 'gt'
  | 'gte'
  | 'lt'
  | 'lte'
  | 'and'
  | 'or'
  | 'not'
  | 'like'
  | 'in'
  | 'inPast';

export type ArithmeticOperator = '+' | '-' | '*' | '/';

export type AggregateOperator =
  | 'min'
  | 'max'
  | 'sum'
  | 'count'
  | 'countDistinct'
  | 'avg'
  | 'avgDistinct';

export type DateOperator = 'day' | 'month' | 'year';
export type TimeOperator = 'hour' | 'minute' | 'second' | 'millisecond';
export type ProjectionOperator = DateOperator | TimeOperator;

export type DateGranularity = 'years' | 'months' | 'weeks' | 'days';
export type TimeGranularity = 'hours' | 'minutes' | 'seconds' | 'millis';
export type DateTimeGranularity = DateGranularity | TimeGranularity;

export type Operator =
  | UnaryOperator
  | BooleanOperator
  | ArithmeticOperator
  | AggregateOperator
  | ProjectionOperator
  | 'ident';

export type Expr = FieldExpr | Scalar | Scalar[];

/**
 *  A tree of expressions, each of which has an associated name.
 */
export abstract class FieldExpr {
  // A human-readable representation of the expression. Use this to refer to the
  // expression in a `select` or `orderBy`.
  name: string;

  // User-specified alias for expression. Can be used in a `select` and then in
  // a subsequent `orderBy`.
  alias?: string;

  constructor(name: string) {
    this.name = name;
  }

  toString(): string {
    return this.name;
  }

  abstract operator(): Operator;
  abstract operands(): Expr[]
}

/**
 * A binary boolean expression. Can be combined with other boolean expressions
 * using `and`, `or` methods.
 */
export class BooleanFieldExpr extends FieldExpr {
  field: FieldExpr;
  op: BooleanOperator;
  other: FieldExpr;

  constructor(
    field: FieldExpr,
    op: BooleanOperator,
    other: FieldExpr
  ) {
    super(`(${field.name} ${op} ${other.name})`);
    this.field = field;
    this.op = op;
    this.other = other;
  }

  operator(): Operator {
    return this.op;
  }

  operands(): Expr[] {
    return [this.field, this.other];
  }

  and(that: BooleanFieldExpr | UnaryBooleanFieldExpr): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'and', that);
  }

  or(that: BooleanFieldExpr | UnaryBooleanFieldExpr): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'or', that);
  }
}

/**
 * A unary boolean expression.
 * E.g., a null check on a field can be expressed using a UnaryBooleanFieldExpr:
 * ```
 * const nameField = new Field<string>('name');
 * const isNameNotNull = new UnaryBooleanFieldExpr(nameField, 'isNotNull');
 * ```
 */
export class UnaryBooleanFieldExpr extends FieldExpr {
  field: FieldExpr;
  op: UnaryOperator;

  constructor(field: FieldExpr, op: UnaryOperator) {
    super(`(${op}(${field.name}))`);
    this.field = field;
    this.op = op;
  }

  operator(): Operator {
    return this.op;
  }

  operands(): Expr[] {
    return [this.field];
  }

  and(that: BooleanFieldExpr | UnaryBooleanFieldExpr): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'and', that);
  }

  or(that: BooleanFieldExpr | UnaryBooleanFieldExpr): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'or', that);
  }
}

/**
 * A field expression to represent an aggregation applied on a field expression.
 * E.g., a sum of a field can be expressed as:
 * ```
 * const price = new Field<number>('price');
 * const totalPrice = new AggregateFieldExpr<number>(price, 'sum');
 * ```
 */
export class AggregateFieldExpr<T> extends FieldExpr {
  private field: FieldExpr;
  private op: AggregateOperator;

  constructor(field: FieldExpr, op: AggregateOperator) {
    super(`(${op}(${field.name}))`);
    this.field = field;
    this.op = op;
  }

  override operator(): Operator {
    return this.op;
  }

  override operands(): Expr[] {
    return [this.field];
  }

  /**
   * Alias this expression. This method is useful when the aggregate expression
   * is defined in a `select` and must be referred to in a subsequent `orderBy`.
   * E.g.,
   * ```
   * let query = MyTable
   *    .select(name, price.sum().as('totalPrice'))
   *    .orderBy(['totalPrice', 'DESC'])
   *    .limit(10);
   * ```
   * @param alias
   * @returns
   */
  as(alias: string): AggregateFieldExpr<T> {
    let copy = new AggregateFieldExpr<T>(this.field, this.op);
    copy.alias = alias;
    return copy;
  }
}

// TODO(PAT-3177): Define ArithmeticFieldExpr?
