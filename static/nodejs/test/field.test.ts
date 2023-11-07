import {
  ArrayField,
  DateField,
  DateTimeField,
  DerivedField,
  Field,
  LiteralField,
  TimeField,
  addDuration,
  ISO_TIME_REGEX,
} from '../src/field';
import { describe, expect, test } from '@jest/globals';
import { AggregateFieldExpr, BooleanFieldExpr } from '../src/field_expr';

describe('Field', () => {
  test('as() returns a copy and does not mutate this', () => {
    const field = new Field<number>('a_number');
    const aliased = field.as('floaty');
    expect(field).not.toBe(aliased);
    expect(field.name).toBe('a_number');
    expect(aliased.name).toBe('a_number');
    expect(field.alias).toBe(undefined);
    expect(aliased.alias).toBe('floaty');
  });

  test('has the expected operator and operands', () => {
    const field = new Field<string>('invoice_id');
    expect(field.operands()).toStrictEqual(['invoice_id']);
    expect(field.operator()).toBe('ident');
  });

  test('returns the correct aggregate expression', () => {
    const price = new Field<number>('price');
    const maxPrice = price.avgDistinct();
    expect(maxPrice instanceof AggregateFieldExpr).toBeTruthy();
    expect(maxPrice.operands()).toStrictEqual([price]);
    expect(maxPrice.operator()).toBe('avgDistinct');

    const totalPrice = price.sum();
    expect(totalPrice instanceof AggregateFieldExpr).toBeTruthy();
    expect(totalPrice.operands()).toStrictEqual([price]);
    expect(totalPrice.operator()).toBe('sum');
  });
});

describe('DerivedField', () => {
  test('as() returns a copy of the derived field and does not mutate this', () => {
    const startedOn = new DateField('startedOn');
    const startedOnYear = startedOn.year;
    const aliased = startedOnYear.as('startedOnYear');
    expect(startedOnYear instanceof DerivedField).toBeTruthy();
    expect(startedOnYear).not.toBe(aliased);
    expect(startedOnYear.name).toBe('(year(startedOn))');
    expect(aliased.name).toBe('(year(startedOn))');
    expect(startedOnYear.alias).toBe(undefined);
    expect(aliased.alias).toBe('startedOnYear');
  });
});

describe('addDuration', () => {
  const now = new Date();

  let want = new Date(now);
  want.setMonth(want.getMonth() + 2);
  expect(addDuration(now, 2, 'months')).toStrictEqual(want);

  want = new Date(now);
  want.setFullYear(want.getFullYear() - 12);
  expect(addDuration(now, -12, 'years')).toStrictEqual(want);

  expect(addDuration(now, -1, 'weeks').getTime() - now.getTime()).toBe(
    -7 * 24 * 60 * 60 * 1000
  );

  expect(addDuration(now, 2, 'days').getTime() - now.getTime()).toBe(
    2 * 24 * 60 * 60 * 1000
  );

  expect(addDuration(now, 24, 'hours').getTime() - now.getTime()).toBe(
    24 * 60 * 60 * 1000
  );

  expect(addDuration(now, 10, 'minutes').getTime() - now.getTime()).toBe(
    10 * 60 * 1000
  );

  expect(addDuration(now, 123, 'milliseconds').getTime() - now.getTime()).toBe(
    123
  );
});

describe('DateField', () => {
  test('boolean methods returns expected BooleanFieldExpression', () => {
    const startedOn = new DateField('startedOn');
    const d = new Date('2023-11-01');

    // Assert operator.
    expect(startedOn.gt(d).operator()).toBe('gt');
    expect(startedOn.lt(d).operator()).toBe('lt');
    expect(startedOn.before(d).operator()).toBe('lt');
    expect(startedOn.after(d).operator()).toBe('gt');
    expect(startedOn.gte(d).operator()).toBe('gte');
    expect(startedOn.lte(d).operator()).toBe('lte');
    expect(startedOn.eq(d).operator()).toBe('eq');
    expect(startedOn.neq(d).operator()).toBe('neq');

    // Assert operands.
    expect(startedOn.gt(d).operands()[0]).toBe(startedOn);
    expect(startedOn.lt(d).operands()[0]).toBe(startedOn);
    expect(startedOn.before(d).operands()[0]).toBe(startedOn);
    expect(startedOn.after(d).operands()[0]).toBe(startedOn);
    expect(startedOn.gte(d).operands()[0]).toBe(startedOn);
    expect(startedOn.lte(d).operands()[0]).toBe(startedOn);
    expect(startedOn.eq(d).operands()[0]).toBe(startedOn);
    expect(startedOn.neq(d).operands()[0]).toBe(startedOn);

    expect(startedOn.gt(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedOn.lt(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(
      startedOn.before(d).operands()[1] instanceof LiteralField
    ).toBeTruthy();
    expect(
      startedOn.after(d).operands()[1] instanceof LiteralField
    ).toBeTruthy();
    expect(startedOn.gte(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedOn.lte(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedOn.eq(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedOn.neq(d).operands()[1] instanceof LiteralField).toBeTruthy();

    expect((startedOn.gt(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-11-01'
    );
    expect((startedOn.lt(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-11-01'
    );
    expect(
      (startedOn.before(d).operands()[1] as LiteralField<string>).value
    ).toBe('2023-11-01');
    expect(
      (startedOn.after(d).operands()[1] as LiteralField<string>).value
    ).toBe('2023-11-01');
    expect((startedOn.gte(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-11-01'
    );
    expect((startedOn.lte(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-11-01'
    );
    expect((startedOn.eq(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-11-01'
    );
    expect((startedOn.neq(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-11-01'
    );
  });

  test('inPast operator is expressed correctly', () => {
    const startedOn = new DateField('startedOn');
    const inPast = startedOn.inPast(1, 2, 'weeks');
    expect(inPast.operator()).toBe('and');
    const operand1 = inPast.operands()[0] as BooleanFieldExpr;
    expect(operand1.operator()).toBe('gte');
    expect(operand1.operands()[0]).toBe(startedOn);

    const operand2 = inPast.operands()[1] as BooleanFieldExpr;
    expect(operand2.operator()).toBe('lte');
    expect(operand2.operands()[0]).toBe(startedOn);

    // NB: Testing the exact RHS values is flaky because it depends on the
    // current time. The tests for `addDuration` should exercise the correctness
    // of the computed time bounds.
    const lowerBound = (operand1.operands()[1] as LiteralField<string>)
      .value as string;
    const upperBound = (operand2.operands()[1] as LiteralField<string>)
      .value as string;
    expect(new Date(lowerBound) <= new Date(upperBound)).toBe(true);
  });
});

describe('TimeField', () => {
  test('boolean methods returns expected BooleanFieldExpression', () => {
    const startedAt = new TimeField('startedAt');
    const t = '13:23:01.123';

    // Assert operator.
    expect(startedAt.gt(t).operator()).toBe('gt');
    expect(startedAt.lt(t).operator()).toBe('lt');
    expect(startedAt.before(t).operator()).toBe('lt');
    expect(startedAt.after(t).operator()).toBe('gt');
    expect(startedAt.gte(t).operator()).toBe('gte');
    expect(startedAt.lte(t).operator()).toBe('lte');
    expect(startedAt.eq(t).operator()).toBe('eq');
    expect(startedAt.neq(t).operator()).toBe('neq');

    // Assert operands.
    expect(startedAt.gt(t).operands()[0]).toBe(startedAt);
    expect(startedAt.lt(t).operands()[0]).toBe(startedAt);
    expect(startedAt.before(t).operands()[0]).toBe(startedAt);
    expect(startedAt.after(t).operands()[0]).toBe(startedAt);
    expect(startedAt.gte(t).operands()[0]).toBe(startedAt);
    expect(startedAt.lte(t).operands()[0]).toBe(startedAt);
    expect(startedAt.eq(t).operands()[0]).toBe(startedAt);
    expect(startedAt.neq(t).operands()[0]).toBe(startedAt);

    expect(startedAt.gt(t).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.lt(t).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(
      startedAt.before(t).operands()[1] instanceof LiteralField
    ).toBeTruthy();
    expect(
      startedAt.after(t).operands()[1] instanceof LiteralField
    ).toBeTruthy();
    expect(startedAt.gte(t).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.lte(t).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.eq(t).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.neq(t).operands()[1] instanceof LiteralField).toBeTruthy();

    expect((startedAt.gt(t).operands()[1] as LiteralField<string>).value).toBe(
      '13:23:01.123'
    );
    expect((startedAt.lt(t).operands()[1] as LiteralField<string>).value).toBe(
      '13:23:01.123'
    );
    expect(
      (startedAt.before(t).operands()[1] as LiteralField<string>).value
    ).toBe('13:23:01.123');
    expect(
      (startedAt.after(t).operands()[1] as LiteralField<string>).value
    ).toBe('13:23:01.123');
    expect((startedAt.gte(t).operands()[1] as LiteralField<string>).value).toBe(
      '13:23:01.123'
    );
    expect((startedAt.lte(t).operands()[1] as LiteralField<string>).value).toBe(
      '13:23:01.123'
    );
    expect((startedAt.eq(t).operands()[1] as LiteralField<string>).value).toBe(
      '13:23:01.123'
    );
    expect((startedAt.neq(t).operands()[1] as LiteralField<string>).value).toBe(
      '13:23:01.123'
    );
  });

  test('inPast operator is expressed correctly', () => {
    const startedAt = new TimeField('startedOn');
    const inPast = startedAt.inPast(1, 2, 'minutes');
    expect(inPast.operator()).toBe('and');
    const operand1 = inPast.operands()[0] as BooleanFieldExpr;
    expect(operand1.operator()).toBe('gte');
    expect(operand1.operands()[0]).toBe(startedAt);

    const operand2 = inPast.operands()[1] as BooleanFieldExpr;
    expect(operand2.operator()).toBe('lte');
    expect(operand2.operands()[0]).toBe(startedAt);

    // NB: Testing the exact RHS values is flaky because it depends on the
    // current time. The tests for `addDuration` should exercise the correctness
    // of the computed time bounds.
    const lowerBound = (operand1.operands()[1] as LiteralField<string>)
      .value as string;
    const upperBound = (operand2.operands()[1] as LiteralField<string>)
      .value as string;
    expect(upperBound).toMatch(ISO_TIME_REGEX);
    expect(lowerBound).toMatch(ISO_TIME_REGEX);
    expect(lowerBound <= upperBound).toBe(true);
  });

  test('inPast operator clamps to time bounds', () => {
    const startedAt = new TimeField('startedOn');
    const inPast = startedAt.inPast(0, 25, 'hours');
    let [operand1, operand2] = inPast.operands() as [
      BooleanFieldExpr,
      BooleanFieldExpr
    ];

    let lowerBound = (operand1.operands()[1] as LiteralField<string>)
      .value as string;
    let upperBound = (operand2.operands()[1] as LiteralField<string>)
      .value as string;
    expect(lowerBound).toBe('00:00:00.000');
    expect(lowerBound <= upperBound).toBe(true);

    const inFuture = startedAt.inPast(-1024, 0, 'hours');
    [operand1, operand2] = inFuture.operands() as [
      BooleanFieldExpr,
      BooleanFieldExpr
    ];

    lowerBound = (operand1.operands()[1] as LiteralField<string>)
      .value as string;
    upperBound = (operand2.operands()[1] as LiteralField<string>)
      .value as string;
    expect(upperBound).toBe('23:59:59.999');
    expect(lowerBound <= upperBound).toBe(true);
  });
});

describe('DateTimeField', () => {
  test('boolean methods returns expected BooleanFieldExpression', () => {
    const startedAt = new DateTimeField('startedAt');
    const d = new Date(
      'Wed Feb 01 2023 13:01:13 GMT-0800 (Pacific Standard Time)'
    );
    const boolExpr = startedAt.lte(d);
    expect(boolExpr instanceof BooleanFieldExpr).toBeTruthy();
    expect(boolExpr.field).toBe(startedAt);
    expect(boolExpr.op).toBe('lte');
    expect(boolExpr.other instanceof LiteralField).toBeTruthy();
    expect((boolExpr.other as LiteralField<string>).value).toBe(
      '2023-02-01T21:01:13.000Z'
    );

    // Assert operator.
    expect(startedAt.gt(d).operator()).toBe('gt');
    expect(startedAt.lt(d).operator()).toBe('lt');
    expect(startedAt.before(d).operator()).toBe('lt');
    expect(startedAt.after(d).operator()).toBe('gt');
    expect(startedAt.gte(d).operator()).toBe('gte');
    expect(startedAt.lte(d).operator()).toBe('lte');
    expect(startedAt.eq(d).operator()).toBe('eq');
    expect(startedAt.neq(d).operator()).toBe('neq');

    // Assert operands.
    expect(startedAt.gt(d).operands()[0]).toBe(startedAt);
    expect(startedAt.lt(d).operands()[0]).toBe(startedAt);
    expect(startedAt.before(d).operands()[0]).toBe(startedAt);
    expect(startedAt.after(d).operands()[0]).toBe(startedAt);
    expect(startedAt.gte(d).operands()[0]).toBe(startedAt);
    expect(startedAt.lte(d).operands()[0]).toBe(startedAt);
    expect(startedAt.eq(d).operands()[0]).toBe(startedAt);
    expect(startedAt.neq(d).operands()[0]).toBe(startedAt);

    expect(startedAt.gt(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.lt(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(
      startedAt.before(d).operands()[1] instanceof LiteralField
    ).toBeTruthy();
    expect(
      startedAt.after(d).operands()[1] instanceof LiteralField
    ).toBeTruthy();
    expect(startedAt.gte(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.lte(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.eq(d).operands()[1] instanceof LiteralField).toBeTruthy();
    expect(startedAt.neq(d).operands()[1] instanceof LiteralField).toBeTruthy();

    expect((startedAt.gt(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-02-01T21:01:13.000Z'
    );
    expect((startedAt.lt(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-02-01T21:01:13.000Z'
    );
    expect(
      (startedAt.before(d).operands()[1] as LiteralField<string>).value
    ).toBe('2023-02-01T21:01:13.000Z');
    expect(
      (startedAt.after(d).operands()[1] as LiteralField<string>).value
    ).toBe('2023-02-01T21:01:13.000Z');
    expect((startedAt.gte(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-02-01T21:01:13.000Z'
    );
    expect((startedAt.lte(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-02-01T21:01:13.000Z'
    );
    expect((startedAt.eq(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-02-01T21:01:13.000Z'
    );
    expect((startedAt.neq(d).operands()[1] as LiteralField<string>).value).toBe(
      '2023-02-01T21:01:13.000Z'
    );
  });

  test('inPast operator is expressed correctly', () => {
    const startedAt = new DateTimeField('startedAt');
    const inPast = startedAt.inPast(1, 2, 'hours');
    expect(inPast.operator()).toBe('and');
    const operand1 = inPast.operands()[0] as BooleanFieldExpr;
    expect(operand1.operator()).toBe('gte');
    expect(operand1.operands()[0]).toBe(startedAt);

    const operand2 = inPast.operands()[1] as BooleanFieldExpr;
    expect(operand2.operator()).toBe('lte');
    expect(operand2.operands()[0]).toBe(startedAt);

    // NB: Testing the exact RHS values is flaky because it depends on the
    // current time. The tests for `addDuration` should exercise the correctness
    // of the computed time bounds.
    const lowerBound = (operand1.operands()[1] as LiteralField<string>)
      .value as string;
    const upperBound = (operand2.operands()[1] as LiteralField<string>)
      .value as string;
    expect(new Date(lowerBound) <= new Date(upperBound)).toBe(true);
  });
});

describe('ArrayField', () => {
  test('method hasAny returns expected BooleanFieldExpression', () => {
    const tagNames = new ArrayField<string>('tagNames');
    const hasAnyTags = tagNames.hasAny(['foo', 'bar']);

    expect(hasAnyTags.operator()).toBe('hasAny');
    expect(hasAnyTags.operands()[0]).toBe(tagNames);
    expect(
      (hasAnyTags.operands()[1] as LiteralField<string>).value
    ).toStrictEqual(['foo', 'bar']);
  });

  test('method hasAll returns expected BooleanFieldExpression', () => {
    const tagNames = new ArrayField<string>('tagNames');
    const hasAllTags = tagNames.hasAll(['foo', 'bar']);

    expect(hasAllTags.operator()).toBe('hasAll');
    expect(hasAllTags.operands()[0]).toBe(tagNames);
    expect(
      (hasAllTags.operands()[1] as LiteralField<string>).value
    ).toStrictEqual(['foo', 'bar']);
  });
});
