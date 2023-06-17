import { Field } from '../src//field';
import {describe, expect, test} from '@jest/globals'
import { AggregateFieldExpr } from '../src/field_expr';

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
  })
});