import { FieldExpr, BooleanFieldExpr, UnaryBooleanFieldExpr, Operator, Expr } from '../src/field_expr';
import { describe, expect, test } from '@jest/globals';
import { AggregateFieldExpr } from '../src/field_expr';

describe('FieldExpr', () => {
    test('should have correct name property', () => {
        class ConcreteFieldExpr extends FieldExpr {
            operator(): Operator {
                return 'ident';
            }

            operands(): Expr[] {
                return [];
            }
        }

        const fieldExpr = new ConcreteFieldExpr('field');
        expect(fieldExpr.name).toBe('field');
    });
});

describe('BooleanFieldExpr', () => {
    test('should have correct field, op, and other properties', () => {
        class ConcreteFieldExpr extends FieldExpr {
            operator(): Operator {
                return 'ident';
            }

            operands(): Expr[] {
                return [];
            }
        }

        const field = new ConcreteFieldExpr('field');
        const other = new ConcreteFieldExpr('other');
        const booleanFieldExpr = new BooleanFieldExpr(field, 'eq', other);
        expect(booleanFieldExpr.field).toBe(field);
        expect(booleanFieldExpr.op).toBe('eq');
        expect(booleanFieldExpr.other).toBe(other);
    });

    test('should return correct operator', () => {
        class ConcreteFieldExpr extends FieldExpr {
            operator(): Operator {
                return 'ident';
            }

            operands(): Expr[] {
                return [];
            }
        }

        const field = new ConcreteFieldExpr('field');
        const other = new ConcreteFieldExpr('other');
        const booleanFieldExpr = new BooleanFieldExpr(field, 'eq', other);
        expect(booleanFieldExpr.operator()).toBe('eq');
    });

    // Add more test cases for the BooleanFieldExpr class here...
});

describe('UnaryBooleanFieldExpr', () => {
    test('should have correct field and op properties', () => {
        class ConcreteFieldExpr extends FieldExpr {
            operator(): Operator {
                return 'ident';
            }

            operands(): Expr[] {
                return [];
            }
        }

        const field = new ConcreteFieldExpr('field');
        const unaryBooleanFieldExpr = new UnaryBooleanFieldExpr(field, 'isNull');
        expect(unaryBooleanFieldExpr.field).toBe(field);
        expect(unaryBooleanFieldExpr.op).toBe('isNull');
    });

    test('should return correct operator', () => {
        class ConcreteFieldExpr extends FieldExpr {
            operator(): Operator {
                return 'ident';
            }

            operands(): Expr[] {
                return [];
            }
        }

        const field = new ConcreteFieldExpr('field');
        const unaryBooleanFieldExpr = new UnaryBooleanFieldExpr(field, 'isNull');
        expect(unaryBooleanFieldExpr.operator()).toBe('isNull');
    });

    // Add more test cases for the UnaryBooleanFieldExpr class here...
});

describe('AggregateFieldExpr', () => {
    test('should have correct field and op properties', () => {
        class ConcreteFieldExpr extends FieldExpr {
            operator(): Operator {
                return 'ident';
            }

            operands(): Expr[] {
                return [];
            }
        }

        const field = new ConcreteFieldExpr('field');
        const aggregateFieldExpr = new AggregateFieldExpr<number>(field, 'max');
        expect(aggregateFieldExpr.operator()).toBe('max');
    });

    test('should return correct operator', () => {
        class ConcreteFieldExpr extends FieldExpr {
            operator(): Operator {
                return 'ident';
            }

            operands(): Expr[] {
                return [];
            }
        }

        const field = new ConcreteFieldExpr('field');
        const aggregateFieldExpr = new AggregateFieldExpr<number>(field, 'max');
        expect(aggregateFieldExpr.operator()).toBe('max');
    });
});
