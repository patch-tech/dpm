from ..field_expr import (
    FieldExpr,
    BooleanFieldExpr,
    UnaryBooleanFieldExpr,
    AggregateFieldExpr,
)

#
# Test FieldExpr
#


def test_field_expr():
    field = FieldExpr("name")
    assert field.name == "name"
    assert field.alias is None

    field_with_alias = FieldExpr("name", "alias")
    assert field_with_alias.name == "name"
    assert field_with_alias.alias == "alias"


def test_field_expr_to_string():
    field = FieldExpr("name")
    assert field.to_string() == "name"


#
# Test BooleanFieldExpr
#


def test_boolean_field_expr():
    field1 = FieldExpr("field1")
    field2 = FieldExpr("field2")
    bool_expr = BooleanFieldExpr(field1, "eq", field2)
    assert bool_expr.field == field1
    assert bool_expr.op == "eq"
    assert bool_expr.other == field2


def test_boolean_field_expr_and():
    field1 = FieldExpr("field1")
    field2 = FieldExpr("field2")
    bool_expr1 = BooleanFieldExpr(field1, "eq", field2)
    bool_expr2 = BooleanFieldExpr(field1, "neq", field2)
    and_expr = bool_expr1 & bool_expr2
    assert and_expr.field == bool_expr1
    assert and_expr.op == "and"
    assert and_expr.other == bool_expr2


def test_boolean_field_expr_or():
    field1 = FieldExpr("field1")
    field2 = FieldExpr("field2")
    bool_expr1 = BooleanFieldExpr(field1, "eq", field2)
    bool_expr2 = BooleanFieldExpr(field1, "neq", field2)
    or_expr = bool_expr1 | bool_expr2
    assert or_expr.field == bool_expr1
    assert or_expr.op == "or"
    assert or_expr.other == bool_expr2


#
# Test UnaryBooleanFieldExpr
#


def test_unary_boolean_field_expr():
    field = FieldExpr("field")
    unary_expr = UnaryBooleanFieldExpr(field, "isNull")
    assert unary_expr.name == "(isNull(field))"
    assert unary_expr.field == field
    assert unary_expr.op == "isNull"


def test_unary_boolean_expr_and():
    field = FieldExpr("field")
    unary_expr = UnaryBooleanFieldExpr(field, "isNotNull")
    and_expr = unary_expr & field
    assert and_expr.field == unary_expr
    assert and_expr.op == "and"
    assert and_expr.other == field


def test_unary_boolean_expr_or():
    field = FieldExpr("field")
    unary_expr = UnaryBooleanFieldExpr(field, "isNotNull")
    or_expr = unary_expr | field
    assert or_expr.field == unary_expr
    assert or_expr.op == "or"
    assert or_expr.other == field


#
# Test AggregateFieldExpr
#


def test_aggregate_field_expr():
    field = FieldExpr("field")
    agg_expr = AggregateFieldExpr(field, "min")
    assert agg_expr.name == "(min(field))"
    assert agg_expr.field == field
    assert agg_expr.op == "min"


def test_aggregate_field_expr_with_alias():
    field = FieldExpr("field")
    agg_expr = AggregateFieldExpr(field, "count")
    agg_expr_with_alias = agg_expr.with_alias("alias")
    assert agg_expr_with_alias.name == "(count(field))"
    assert agg_expr_with_alias.field == field
    assert agg_expr_with_alias.op == "count"
    assert agg_expr_with_alias.alias == "alias"
