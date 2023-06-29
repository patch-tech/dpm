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
  Scalar,
  UnaryBooleanFieldExpr,
} from './field_expr';

/**
 * A base class to represent a field in a `Table`. Identifies the underlying DB
 * table column by its name.
 */
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

  /**
   * Alias this field.
   * E.g.,
   * ```
   * let query = MyTable
   *    .select(fieldWithLongName.as('shortName'), price)
   *    .orderBy(['shortName', 'DESC'])
   *    .limit(10);
   * ```
   * @param alias
   * @returns
   */
  as(alias: string): Field<T> {
    let copy = new Field<T>(this.name);
    copy.alias = alias;
    return copy;
  }

  private asBooleanExpr(
    op: BooleanOperator,
    that: T | T[] | Field<T>
  ): BooleanFieldExpr {
    let that_ = that instanceof Field ? that : new LiteralField(that);
    return new BooleanFieldExpr(this, op, that_);
  }

  /**
   * Returns an `max` aggregation applied on this field.
   */
  max(): AggregateFieldExpr<T> {
    return new AggregateFieldExpr<T>(this, 'max');
  }

  /**
   * Returns an `min` aggregation applied on this field.
   */
  min(): AggregateFieldExpr<T> {
    return new AggregateFieldExpr<T>(this, 'min');
  }

  /**
   * Returns an `count` aggregation applied on this field.
   */
  count(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'count');
  }

  /**
   * Returns a distinct `count` aggregation applied on this field.
   */
  countDistinct(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'countDistinct');
  }

  /**
   * Returns an `average` aggregation applied on this field.
   */
  avg(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'avg');
  }

  /**
   * Returns a distinct `average` aggregation applied on this field.
   */
  avgDistinct(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'avgDistinct');
  }

  /**
   * Returns a boolean expression with an equality check.
   */
  eq(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('eq', that);
  }

  /**
   * Returns a boolean expression with a not equal check.
   */
  neq(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('neq', that);
  }

  /**
   * Returns a boolean expression with greater than (>) check.
   */
  gt(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('gt', that);
  }

  /**
   * Returns a boolean expression with greater than or equal (>=) check.
   */
  gte(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('gte', that);
  }

  /**
   * Returns a boolean expression with lesser than (<) check.
   */
  lt(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('lt', that);
  }

  /**
   * Returns a boolean expression with lesser than or equal (<=) check.
   */
  lte(that: T | Field<T>): BooleanFieldExpr {
    return this.asBooleanExpr('lte', that);
  }

  /**
   * Returns a boolean expression with an array membership check. The field's
   * value must exactly match at least one entry in `that` for the check to be
   * true.
   */
  in(that: T[]): BooleanFieldExpr {
    return this.asBooleanExpr('in', that);
  }

  /**
   * Returns a boolean expression that checks if the field's value is in between
   * a range (inclusive of bounds).
   */
  between(minVal: T, maxVal: T): BooleanFieldExpr {
    return this.gte(minVal).and(this.lte(maxVal));
  }

  /**
   * Returns a boolean expression that checks if the field is null.
   */
  isNull(): UnaryBooleanFieldExpr {
    return new UnaryBooleanFieldExpr(this, 'isNull');
  }

  /**
   * Returns a boolean expression that checks if the field is not null.
   */
  isNotNull(): UnaryBooleanFieldExpr {
    return new UnaryBooleanFieldExpr(this, 'isNotNull');
  }
}

/**
 * A literal field value. Used to represent constant RHS values in expressions.
 */
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

/**
 * A string field. Defines additional operators that are specific to strings.
 */
export class StringField extends Field<string> {
  constructor(name: string) {
    super(name);
  }

  /**
   * Returns a boolean expression for a string `like` check.
   * See: https://en.wikibooks.org/wiki/Structured_Query_Language/Like_Predicate#LIKE
   * E.g.,
   * ```
   * let query = MyTable
   *    .select(name, price)
   *    .filter(name.like('%shirt%'))
   *    .limit(10);
   * ```
   * @param pattern The like pattern with wildcards: % (one or more) and _ (exactly one).
   * @returns
   */
  like(pattern: string): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'like', new LiteralField(pattern));
  }
}

/**
 * A derived field obtained by applying a projection operator.
 * E.g.
 * ```
 * const startDateTime = new DateTimeField('started_at');
 * const startYear = new DerivedField<number, Date>(startDateTime, 'year');
 * ```
 *
 * @see {@link DateField#year}, {@link DateField#month}, {@link DateField#day}
 * for getters that return derived fields.
 */
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
 * @param d
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

  /**
   * Projects the date to its month.
   */
  get month(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'month');
  }

  /**
   * Projects the date to its day.
   */
  get day(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'day');
  }

  /**
   * Projects the date to its year.
   */
  get year(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'year');
  }

  /**
   * Returns a boolean expression that checks if this date is before `d`.
   * @param d
   */
  before(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(
      this,
      'lt',
      new LiteralField(toISODateString(d))
    );
  }

  /**
   * Returns a boolean expression that checks if this date is after `d`.
   * @param d
   */
  after(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(
      this,
      'gt',
      new LiteralField(toISODateString(d))
    );
  }

  // TODO(PAT-3290): Implement eq, neq, lte, gte.

  /**
   * Returns a boolean expression that checks if this date is less than `d`.
   * @param d
   */
  override lt(d: Date): BooleanFieldExpr {
    return this.before(d);
  }

  /**
   * Returns a boolean expression that checks if this date is greater than `d`.
   * @param d
   */
  override gt(d: Date): BooleanFieldExpr {
    return this.after(d);
  }

  /**
   * Returns a boolean expression that performs a relative range check of this date.
   * The range is specified by its two bounds and a granularity.
   * E.g., the filter expression below checks if the value of `startDate` lies
   * in the past 2 to 3 weeks.
   * ```
   * let query = MyTable
   *    .select(startDate, name)
   *    .filter(startDate.inPast(2, 3, 'weeks'))
   *
   * ```
   * @param olderThan
   * @param newerThan
   * @param granularity
   */
  inPast(
    olderThan: number,
    newerThan: number,
    granularity: DateGranularity
  ): BooleanFieldExpr {
    if (olderThan > newerThan) {
      console.warn(
        `inPast specified with olderThan(${olderThan}) > newerThan(${newerThan}), swapped arguments.`
      );
      [olderThan, newerThan] = [newerThan, olderThan];
    }
    // TODO(PAT-3355): Generate the relative datetime ranges and use the `between` operation.
    return new BooleanFieldExpr(
      this,
      'inPast',
      new LiteralField([olderThan, newerThan, granularity])
    );
  }
}

export class DateTimeField extends DateField {
  constructor(name: string) {
    super(name);
  }

  /**
   * Projects the time to its hour.
   */
  get hour(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'hour');
  }

  /**
   * Projects the time to its minute.
   */
  get minute(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'minute');
  }

  /**
   * Projects the time to its second.
   */
  get second(): DerivedField<number, Date> {
    return new DerivedField<number, Date>(this, 'second');
  }

  // TODO(PAT-3291): Enable millisecond granularity once its available in the Dataset API.
  // TODO(PAT-3290): Implement eq, neq, lte, gte.

  /**
   * Returns a boolean expression that checks if this datetime is before `d`.
   * @param d
   */
  override before(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'lt', new LiteralField(d.toISOString()));
  }

  /**
   * Returns a boolean expression that checks if this datetime is after `d`.
   * @param d
   */
  override after(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'gt', new LiteralField(d.toISOString()));
  }

  /**
   * Returns a boolean expression that performs a relative range check of this datetime.
   * The range is specified by its two bounds and a granularity.
   * E.g., the filter expression below checks if the value of `startDateTime` lies
   * in the past 2 to 3 hours.
   * ```
   * let query = MyTable
   *    .select(startDateTime, name)
   *    .filter(startDateTime.inPast(2, 3, 'hours'))
   *
   * ```
   * @param olderThan
   * @param newerThan
   * @param granularity
   */
  override inPast(
    olderThan: number,
    newerThan: number,
    granularity: DateTimeGranularity
  ): BooleanFieldExpr {
    if (olderThan > newerThan) {
      console.warn(
        `inPast specified with olderThan(${olderThan}) > newerThan(${newerThan}), swapped arguments.`
      );
      [olderThan, newerThan] = [newerThan, olderThan];
    }
    // TODO(PAT-3355): Generate the relative datetime ranges and use the `between` operation.
    return new BooleanFieldExpr(
      this,
      'inPast',
      new LiteralField([olderThan, newerThan, granularity])
    );
  }
}
