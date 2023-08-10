import { FieldExpr, BooleanFieldExpr, UnaryBooleanFieldExpr, Operator, Expr } from '../src/field_expr';
import { describe, expect, test } from '@jest/globals';
import { AggregateFieldExpr } from '../src/field_expr';

class ConcreteFieldExpr extends FieldExpr {
    operator(): Operator {
        return 'ident';
    }

    operands(): Expr[] {
        return [];
    }
}

const field = new ConcreteFieldExpr('field');

describe('FieldExpr', () => {
    test('should have correct name property', () => {
        expect(field.name).toBe('field');
    });
});

const other = new ConcreteFieldExpr('other');
const booleanFieldExpr = new BooleanFieldExpr(field, 'eq', other);

describe('BooleanFieldExpr', () => {
    test('should have correct field, op, and other properties', () => {
        expect(booleanFieldExpr.field).toBe(field);
        expect(booleanFieldExpr.op).toBe('eq');
        expect(booleanFieldExpr.other).toBe(other);
    });

    test('should return correct operator', () => {
        expect(booleanFieldExpr.operator()).toBe('eq');
    });

});

const unaryBooleanFieldExpr = new UnaryBooleanFieldExpr(field, 'isNull');
const aggregateFieldExpr = new AggregateFieldExpr<number>(field, 'max');

describe('UnaryBooleanFieldExpr', () => {
    test('should have correct field and op properties', () => {
        expect(unaryBooleanFieldExpr.field).toBe(field);
        expect(unaryBooleanFieldExpr.op).toBe('isNull');
    });

    test('should return correct operator', () => {
        expect(unaryBooleanFieldExpr.operator()).toBe('isNull');
    });

});

describe('AggregateFieldExpr', () => {
    test('should have correct field and op properties', () => {
        expect(aggregateFieldExpr.operator()).toBe('max');
    });
});
