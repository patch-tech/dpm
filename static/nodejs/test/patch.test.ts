import { LiteralField, StringField, Field } from '../src/field';
import { snakeToCamel, fieldAsGraphQL, selectionAsGraphQL, withAlias, formatDefault } from '../src/backends/patch'
import { expect, test } from '@jest/globals';
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
    const price = new Field<number>("price");
    const size = new StringField("size_of_beans");
    const maxPrice = new AggregateFieldExpr(price, "max");
    expect(fieldAsGraphQL(price)).toBe("price");
    expect(fieldAsGraphQL(size)).toBe("sizeOfBeans");
    expect(fieldAsGraphQL(maxPrice)).toBe("priceMax");
});

test('format', () => {
    const price = new Field<number>("price");
    const threshold = new LiteralField(19.99);

    const expected_default_output = `
     {
    price: {
      lt: 19.99
    }
  }
    `;
    expect(formatDefault("lt", price, threshold)).toBe(expected_default_output.trim());
});