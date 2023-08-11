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

describe('FieldExpr', () => {
    test('should have correct name property', () => {
        const field = new ConcreteFieldExpr('field');

        expect(field.name).toBe('field');
    });
});

describe('BooleanFieldExpr', () => {
    test('should have correct field, op, and other properties', () => {
        const field = new ConcreteFieldExpr('field');
        const other = new ConcreteFieldExpr('other');
        const booleanFieldExpr = new BooleanFieldExpr(field, 'eq', other);

        expect(booleanFieldExpr.field).toBe(field);
        expect(booleanFieldExpr.op).toBe('eq');
        expect(booleanFieldExpr.other).toBe(other);
    });

    test('should return correct operator', () => {
        const field = new ConcreteFieldExpr('field');
        const other = new ConcreteFieldExpr('other');
        const booleanFieldExpr = new BooleanFieldExpr(field, 'eq', other);

        expect(booleanFieldExpr.operator()).toBe('eq');
    });

});

describe('UnaryBooleanFieldExpr', () => {
    test('should have correct field and op properties', () => {
        const field = new ConcreteFieldExpr('field');
        const unaryBooleanFieldExpr = new UnaryBooleanFieldExpr(field, 'isNull');

        expect(unaryBooleanFieldExpr.field).toBe(field);
        expect(unaryBooleanFieldExpr.op).toBe('isNull');
    });

    test('should return correct operator', () => {
        const field = new ConcreteFieldExpr('field');
        const unaryBooleanFieldExpr = new UnaryBooleanFieldExpr(field, 'isNull');

        expect(unaryBooleanFieldExpr.operator()).toBe('isNull');
    });

});

describe('AggregateFieldExpr', () => {
    test('should have correct field and op properties', () => {
        const field = new ConcreteFieldExpr('field');
        const aggregateFieldExpr = new AggregateFieldExpr<number>(field, 'max');

        expect(aggregateFieldExpr.operator()).toBe('max');
    });
});
