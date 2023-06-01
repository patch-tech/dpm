export type Scalar =
  | string
  | number
  | boolean
  | Date;

export type UnaryOperator = 'not' | '-';
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

export type Expr = FieldExpr | Scalar;

/**
 *  A tree of expressions, each of which has an associated name.
 */
export abstract class FieldExpr {
  // A human-readable representation of the expression. Use this to refer to the
  // expression in a `select` or `orderBy`.
  name: string;

  // User-specified alias for expression.
  alias?: string;

  constructor(name: string) {
    this.name = name;
  }

  as(alias: string): FieldExpr {
    this.alias = alias;
    return this;
  }

  toString(): string {
    return this.name;
  }

  abstract operator(): Operator;
  abstract operands(): Expr[]
}

export class UnaryFieldExpr extends FieldExpr {
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
}

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

  and(that: FieldExpr): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'and', that);
  }

  or(that: FieldExpr): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'or', that);
  }

  not(): FieldExpr {
    return new UnaryFieldExpr(this, 'not');
  }
}

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

  override as(alias: string): AggregateFieldExpr<T> {
    super.as(alias);
    return this
  }
}

// TODO(PAT-3177): Define ArithmeticFieldExpr?
