import {
  AggregateFieldExpr,
  BooleanFieldExpr,
  BooleanOperator,
  DateGranularity,
  DateTimeGranularity,
  Expr,
  FieldExpr,
  Operator,
  ProjectionOperator,
  Scalar
} from './field_expr';

export class Field<T extends Scalar> extends FieldExpr {
  private val?: T;

  constructor(name: string) {
    super(name);
  }

  operator(): Operator {
    return 'ident';
  }

  operands(): Expr[] {
    return [this.name];
  }

  as(alias: string): Field<T> {
    let copy = new Field<T>(this.name);
    copy.alias = alias;
    return copy;
  }

  private asBooleanExpr(op: BooleanOperator, that: T | T[] | Field<T>): BooleanFieldExpr {
    let that_ = that instanceof Field ? that : new LiteralField(that);
    return new BooleanFieldExpr(this, op, that_);
  }

  max(): AggregateFieldExpr<T> {
    return new AggregateFieldExpr<T>(this, 'max');
  }

  min(): AggregateFieldExpr<T> {
    return new AggregateFieldExpr<T>(this, 'min');
  }

  count(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'count');
  }

  countDistinct(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'countDistinct');
  }

  avg(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'avg');
  }

  avgDistinct(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'avgDistinct');
  }

  eq(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('eq', that);
  }

  neq(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('neq', that);
  }

  gt(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('gt', that);
  }

  gte(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('gte', that);
  }

  lt(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('lt', that);
  }

  lte(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('lte', that);
  }

  in(that: T[]): BooleanFieldExpr {
    return this.asBooleanExpr('in', that);
  }

  between(minVal: T, maxVal: T): BooleanFieldExpr {
    return this.gte(minVal).and(this.lte(maxVal));
  }
}

export class LiteralField<T extends Scalar> extends Field<T> {
  value: T | T[];

  constructor(value: T | T[]) {
    super(`lit(${value})`);
    this.value = value;
  }

  override operator(): Operator {
    return 'ident';
  }

  override operands(): Expr[] {
    if (Array.isArray(this.value)) {
      return this.value;
    }
    return [this.value];
  }

  override max(): never {
    throw new SyntaxError('Cannot call max on literal field');
  }

  override min(): never {
    throw new SyntaxError('Cannot call min on literal field');
  }

  override count(): never {
    throw new SyntaxError('Cannot call count on literal field');
  }

  override countDistinct(): never {
    throw new SyntaxError('Cannot call countDistinct on literal field');
  }

  override avg(): never {
    throw new SyntaxError('Cannot call avg on literal field');
  }

  override avgDistinct(): never {
    throw new SyntaxError('Cannot call avgDistinct on literal field');
  }
}

export class StringField extends Field<string> {
  constructor(name: string) {
    super(name);
  }

  like(pattern: string): BooleanFieldExpr {
    return new BooleanFieldExpr(
      this,
      'like',
      new LiteralField(pattern)
    );
  }
}

export class DerivedField<T extends Scalar, U extends Scalar> extends Field<T> {
  private op: ProjectionOperator;
  private field: Field<U>;

  constructor(field: Field<U>, op: ProjectionOperator) {
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

  as(alias: string): DerivedField<T, U> {
    let copy = new DerivedField<T, U>(this.field, this.op);
    copy.alias = alias;
    return copy;
  }
}

/**
 * Returns date formatted as YYYY-MM-DD.
 * See RFC3339 and https://www.w3.org/TR/NOTE-datetime
 * @param d Date
 * @returns The date formatted as YYYY-MM-DD
 */
function toISODateString(d: Date): string {
  // .toISOString() returns YYYY-MM-DDTHH:mm:ss.sssZ, extract the date from it.
  // See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString
  return d.toISOString().substring(0, 10);
}

export class DateField extends Field<Date> {
  constructor(name: string) {
    super(name);
  }

  get month(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'month');
  }

  get day(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'day');
  }

  get year(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'year');
  }

  before(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'lt', new LiteralField(toISODateString(d)));
  }

  after(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'gt', new LiteralField(toISODateString(d)));
  }

  // TODO(PAT-3290): Implement eq, neq, lte, gte.

  override lt(d: Date): BooleanFieldExpr {
    return this.before(d);
  }

  override gt(d: Date): BooleanFieldExpr {
    return this.after(d);
  }

  inPast(olderThan: number, newerThan: number, granularity: DateGranularity): BooleanFieldExpr {
    if (olderThan > newerThan) {
      console.warn(`inPast specified with olderThan(${olderThan}) > newerThan(${newerThan}), swapped arguments.`);
      [olderThan, newerThan] = [newerThan, olderThan];
    }
    // TODO(PAT-3355): Generate the relative datetime ranges and use the `between` operation.
    return new BooleanFieldExpr(this, 'inPast', new LiteralField([olderThan, newerThan, granularity]));
  }
}

export class DateTimeField extends DateField {
  constructor(name: string) {
    super(name);
  }

  get hour(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'hour');
  }

  get minute(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'minute');
  }

  get second(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'second');
  }

  // TODO(PAT-3291): Enable millisecond granularity once its available in the Dataset API.

  // TODO(PAT-3290): Implement eq, neq, lte, gte.

  override before(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'lt', new LiteralField(d.toISOString()));
  }

  override after(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'gt', new LiteralField(d.toISOString()));
  }

  override inPast(olderThan: number, newerThan: number, granularity: DateTimeGranularity): BooleanFieldExpr {
    if (olderThan > newerThan) {
      console.warn(`inPast specified with olderThan(${olderThan}) > newerThan(${newerThan}), swapped arguments.`);
      [olderThan, newerThan] = [newerThan, olderThan];
    }
    // TODO(PAT-3355): Generate the relative datetime ranges and use the `between` operation.
    return new BooleanFieldExpr(this, 'inPast', new LiteralField([olderThan, newerThan, granularity]));
  }
}
