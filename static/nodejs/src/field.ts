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
  TimeGranularity,
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
   * Returns a `sum` aggregation applied on this field.
   */
  sum(): AggregateFieldExpr<number> {
    return new AggregateFieldExpr<number>(this, 'sum');
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

  override sum(): never {
    throw new SyntaxError('Cannot call sum on literal field');
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

/**
 * Returns time formatted as HH:mm:ss.sss
 * See RFC3339 and https://www.w3.org/TR/NOTE-datetime
 * @param d
 * @returns The time formatted as HH:mm:ss.sss
 */
function toISOTimeString(d: Date): string {
  // .toISOString() returns YYYY-MM-DDTHH:mm:ss.sssZ, extract the time from it.
  // See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString
  return d.toISOString().substring(11, 23);
}

export const ISO_TIME_REGEX = /^([0-1]\d|2[0-3]):[0-5]\d:[0-5]\d(\.\d\d\d)?$/;

/**
 * Returns whether t is a valid time string, i.e. matches HH:mm:ss[.sss]
 * @param t Time string.
 * @returns Whether t is a valid time string, i.e. matches HH:mm:ss[.sss]
 */
function isValidTimeString(t: string): boolean {
  return t.match(ISO_TIME_REGEX) !== null;
}

const MILLIS_PER_SECOND = 1000;
const MILLIS_PER_MINUTE = 60 * MILLIS_PER_SECOND;
const MILLIS_PER_HOUR = 60 * MILLIS_PER_MINUTE;
const MILLIS_PER_DAY = 24 * MILLIS_PER_HOUR;

/**
 * Adds the specified amount of time to Date 'd'.
 * E.g.,
 * 1. addDuration(d, 8, 'weeks') returns the date 8 weeks in the future from 'd'.
 * 2. addDuration(d, -18, 'years') returns the date 18 years in the past from 'd'.
 * NB: This function is exported so we can test it.
 * @param d Input date.
 * @param n Integral units of time to add.
 * @param granularity The granularity of time units to add.
 * @returns Date offset from 'd' by n units of granularity
 */
export function addDuration(
  d: Date,
  n: number,
  granularity: DateTimeGranularity
): Date {
  let result = new Date(d);
  switch (granularity) {
    case 'years':
      result.setFullYear(result.getFullYear() + n);
      break;
    case 'months':
      result.setMonth(result.getMonth() + n);
      break;
    case 'weeks':
      result.setTime(result.getTime() + n * 7 * MILLIS_PER_DAY);
      break;
    case 'days':
      result.setTime(result.getTime() + n * MILLIS_PER_DAY);
      break;
    case 'hours':
      result.setTime(result.getTime() + n * MILLIS_PER_HOUR);
      break;
    case 'minutes':
      result.setTime(result.getTime() + n * MILLIS_PER_MINUTE);
      break;
    case 'seconds':
      result.setTime(result.getTime() + n * MILLIS_PER_SECOND);
      break;
    case 'milliseconds':
      result.setTime(result.getTime() + n);
      break;
  }
  return result;
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
   * Returns a boolean expression that checks if this date is equal to `d`.
   * @param d
   */
  override eq(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(
      this,
      'eq',
      new LiteralField(toISODateString(d))
    );
  }

  /**
   * Returns a boolean expression that checks if this date is not equal to `d`.
   * @param d
   */
  override neq(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(
      this,
      'neq',
      new LiteralField(toISODateString(d))
    );
  }

  /**
   * Returns a boolean expression that checks if this date is less than or equal to `t`.
   * @param d
   */
  override lte(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(
      this,
      'lte',
      new LiteralField(toISODateString(d))
    );
  }

  /**
   * Returns a boolean expression that checks if this date is greater than `t` or equal to `t`.
   * @param d
   */
  override gte(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(
      this,
      'gte',
      new LiteralField(toISODateString(d))
    );
  }

  /**
   * Returns a boolean expression that performs a relative range check of this date.
   * The range is specified by its two bounds and a granularity.
   * E.g., the filter expression below checks if the value of `startDate` lies
   * in the past 2 to 3 weeks, inclusive of bounds.
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

    let now = new Date();
    let upperBound = addDuration(now, -olderThan, granularity);
    let lowerBound = addDuration(now, -newerThan, granularity);

    return this.gte(lowerBound).and(this.lte(upperBound));
  }
}

export class TimeField extends Field<string> {
  constructor(name: string) {
    super(name);
  }

  private asValidatedBooleanExpr(
    op: BooleanOperator,
    that: string
  ): BooleanFieldExpr {
    if (!isValidTimeString(that)) {
      throw new Error(`Input ${that} must be in format HH:mm:ss[.sss]`);
    }

    return new BooleanFieldExpr(this, op, new LiteralField(that));
  }

  /**
   * Projects the time to its hour.
   */
  get hour(): DerivedField<number, string> {
    return new DerivedField<number, string>(this, 'hour');
  }

  /**
   * Projects the time to its minute.
   */
  get minute(): DerivedField<number, string> {
    return new DerivedField<number, string>(this, 'minute');
  }

  /**
   * Projects the time to its second.
   */
  get second(): DerivedField<number, string> {
    return new DerivedField<number, string>(this, 'second');
  }

  // TODO(PAT-3291): Enable millisecond granularity once its available in the Dataset API.

  /**
   * Returns a boolean expression that checks if this time is before `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  before(t: string): BooleanFieldExpr {
    return this.asValidatedBooleanExpr('lt', t);
  }

  /**
   * Returns a boolean expression that checks if this time is after `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  after(t: string): BooleanFieldExpr {
    return this.asValidatedBooleanExpr('gt', t);
  }

  /**
   * Returns a boolean expression that checks if this time is less than `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  override lt(t: string): BooleanFieldExpr {
    return this.before(t);
  }

  /**
   * Returns a boolean expression that checks if this time is greater than `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  override gt(t: string): BooleanFieldExpr {
    return this.after(t);
  }

  /**
   * Returns a boolean expression that checks if this time is equal to `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  override eq(t: string): BooleanFieldExpr {
    return this.asValidatedBooleanExpr('eq', t);
  }

  /**
   * Returns a boolean expression that checks if this time is not equal to `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  override neq(t: string): BooleanFieldExpr {
    return this.asValidatedBooleanExpr('neq', t);
  }

  /**
   * Returns a boolean expression that checks if this time is less than or equal
   * to `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  override lte(t: string): BooleanFieldExpr {
    return this.asValidatedBooleanExpr('lte', t);
  }

  /**
   * Returns a boolean expression that checks if this time is greater than `t`
   * or equal to `t`.
   * @param t Time string formatted as "HH:mm:ss[.sss]"
   */
  override gte(t: string): BooleanFieldExpr {
    return this.asValidatedBooleanExpr('gte', t);
  }

  /**
   * Returns a boolean expression that performs a relative range check of this time.
   * The range is specified by its two bounds and a granularity.
   * E.g., the filter expression below checks if the value of `startTime` lies
   * in the past 2 to 3 hours, inclusive of bounds.
   * The bounds are clamped to the range [00:00:00, 23:59:59.999]
   * ```
   * let query = MyTable
   *    .select(startTime, name)
   *    .filter(startTime.inPast(2, 3, 'hours'))
   *
   * ```
   * @param olderThan
   * @param newerThan
   * @param granularity
   */
  inPast(
    olderThan: number,
    newerThan: number,
    granularity: TimeGranularity
  ): BooleanFieldExpr {
    if (olderThan > newerThan) {
      console.warn(
        `inPast specified with olderThan(${olderThan}) > newerThan(${newerThan}), swapped arguments.`
      );
      [olderThan, newerThan] = [newerThan, olderThan];
    }

    // We use a Date to perform time arithmetic, and clamp to the ends of the
    // day.
    let now = new Date();
    let upperBound = addDuration(now, -olderThan, granularity);
    // Clamp to end of day on overflow.
    if (toISODateString(upperBound) > toISODateString(now)) {
      upperBound = new Date(now);
      upperBound.setUTCHours(23);
      upperBound.setUTCMinutes(59);
      upperBound.setUTCSeconds(59);
      upperBound.setUTCMilliseconds(999);
    }
    const upperBoundStr = toISOTimeString(upperBound);

    let lowerBound = addDuration(now, -newerThan, granularity);
    // Clamp to start of day on underflow.
    if (toISODateString(lowerBound) < toISODateString(now)) {
      lowerBound = new Date(now);
      lowerBound.setUTCHours(0);
      lowerBound.setUTCMinutes(0);
      lowerBound.setUTCSeconds(0);
      lowerBound.setUTCMilliseconds(0);
    }
    const lowerBoundStr = toISOTimeString(lowerBound);

    return this.gte(lowerBoundStr).and(this.lte(upperBoundStr));
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
   * Returns a boolean expression that checks if this datetime is equal to `d`.
   * @param d
   */
  override eq(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'eq', new LiteralField(d.toISOString()));
  }

  /**
   * Returns a boolean expression that checks if this datetime is not equal to `d`.
   * @param d
   */
  override neq(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'neq', new LiteralField(d.toISOString()));
  }

  /**
   * Returns a boolean expression that checks if this datetime is less than or
   * equal to `t`.
   * @param d
   */
  override lte(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'lte', new LiteralField(d.toISOString()));
  }

  /**
   * Returns a boolean expression that checks if this datetime is greater than
   * `t` or equal to `t`.
   * @param d
   */
  override gte(d: Date): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'gte', new LiteralField(d.toISOString()));
  }

  /**
   * Returns a boolean expression that performs a relative range check of this datetime.
   * The range is specified by its two bounds and a granularity.
   * E.g., the filter expression below checks if the value of `startDateTime` lies
   * in the past 2 to 3 hours, inclusive of bounds.
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

    let now = new Date();
    let upperBound = addDuration(now, -olderThan, granularity);
    let lowerBound = addDuration(now, -newerThan, granularity);

    return this.gte(lowerBound).and(this.lte(upperBound));
  }
}

export class ArrayField<T extends Scalar> extends Field<T> {
  constructor(name: string){
    super(name);
  }

  hasAny(vals: T[]): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'hasAny', new LiteralField(vals));
  }

  hasAll(vals: T[]): BooleanFieldExpr {
    return new BooleanFieldExpr(this, 'hasAll', new LiteralField(vals));
  }

  // Override and disallow operators that cannot be supported.
  override max(): never {
    throw new SyntaxError('Cannot call max on array field');
  }

  override min(): never {
    throw new SyntaxError('Cannot call min on array field');
  }

  override sum(): never {
    throw new SyntaxError('Cannot call sum on array field');
  }

  override count(): never {
    throw new SyntaxError('Cannot call count on array field');
  }

  override countDistinct(): never {
    throw new SyntaxError('Cannot call countDistinct on array field');
  }

  override avg(): never {
    throw new SyntaxError('Cannot call avg on array field');
  }

  override avgDistinct(): never {
    throw new SyntaxError('Cannot call avgDistinct on array field');
  }

  override eq(_that: T | Field<T>): never {
    throw new SyntaxError('Cannot call eq on array field');
  }

  override neq(_that: T | Field<T>): never {
    throw new SyntaxError('Cannot call neq on array field');
  }

  override gt(_that: T | Field<T>): never {
    throw new SyntaxError('Cannot call gt on array field');
  }

  override gte(_that: T | Field<T>): never {
    throw new SyntaxError('Cannot call gte on array field');
  }

  override lt(_that: T | Field<T>): never {
    throw new SyntaxError('Cannot call lt on array field');
  }

  override lte(_that: T | Field<T>): never {
    throw new SyntaxError('Cannot call lte on array field');
  }

  override in(_that: T[]): never {
    throw new SyntaxError('Cannot call in on array field');
  }

  override between(_minVal: T, _maxVal: T): never {
    throw new SyntaxError('Cannot call between on array field');
  }
}
