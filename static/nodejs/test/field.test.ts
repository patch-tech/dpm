import {
  DateField,
  DateTimeField,
  DerivedField,
  Field,
  LiteralField,
  TimeField,
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
});
