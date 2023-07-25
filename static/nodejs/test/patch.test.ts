import { BooleanFieldExpr, UnaryBooleanFieldExpr, Operator, Expr } from '../src/field_expr';
import { LiteralField, StringField, DerivedField, Field } from '../src/field';
import { snakeToCamel, fieldAsGraphQL, selectionAsGraphQL, withAlias, formatDefault } from '../src/backends/patch'
import { describe, expect, test } from '@jest/globals';
import { AggregateFieldExpr } from '../src/field_expr';

test('snakeToCamel', () => {
    expect(snakeToCamel("hello_world")).toBe("helloWorld");
    expect(snakeToCamel("HELLO_WORLD")).toBe("helloWorld");
    expect(snakeToCamel("hello_WORLD")).toBe("helloWorld");
    expect(snakeToCamel("hello")).toBe("hello");
});


test('withAlias', () => {
    expect(withAlias("field")).toBe("field");
    expect(withAlias("field", "field_alias")).toBe("field_alias: field");
    expect(withAlias("field", "field")).toBe("field: field");
});

test('selectionAsGraphql', () => {
    const price = new Field("price");
    const size = new Field("size");
    expect(selectionAsGraphQL([price, size])).toBe("price\nsize");
});

test('fieldAsGraphql', () => {
    const price = new LiteralField("price");
    const size = new StringField("size_of_beans");
    const maxPrice = new AggregateFieldExpr(price, "max");
    expect(fieldAsGraphQL(price)).toBe("\"price\"");
    expect(fieldAsGraphQL(size)).toBe("sizeOfBeans");
    expect(fieldAsGraphQL(maxPrice)).toBe("\"price\"Max");
});

test('format', () => {
    const price = new LiteralField("price");
    const size = new LiteralField("size_of_beans");

    const expected_default_output = `
     {
    "price": {
      and: "size_of_beans"
    }
  }
    `;
    expect(formatDefault("and", price, size)).toBe(expected_default_output.trim());
});