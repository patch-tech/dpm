from datetime import date, time, datetime

from ..field import (
    DateField,
    TimeField,
    DateTimeField,
    DerivedField,
    Field,
    AggregateFieldExpr,
    LiteralField,
)


def test_field_as_returns_copy_and_does_not_mutate_this():
    field = Field("a_number")
    aliased = field.with_alias("floaty")
    assert field is not aliased
    assert field.name == "a_number"
    assert aliased.name == "a_number"
    assert field.alias is None
    assert aliased.alias == "floaty"


def test_field_has_the_expected_operator_and_operands():
    field = Field("invoice_id")
    assert field.operands() == ["invoice_id"]
    assert field.operator() == "ident"


def test_field_returns_the_correct_aggregate_expression():
    price = Field("price")
    max_price = price.avg_distinct()
    assert isinstance(max_price, AggregateFieldExpr)
    assert max_price.operands() == [price]
    assert max_price.operator() == "avgDistinct"

    total_price = price.sum()
    assert isinstance(total_price, AggregateFieldExpr)
    assert total_price.operands() == [price]
    assert total_price.operator() == "sum"


def test_derived_field_as_returns_copy_of_derived_field_and_does_not_mutate_this():
    started_on = DateField("startedOn")
    started_on_year = started_on.year
    aliased = started_on_year.with_alias("startedOnYear")
    assert isinstance(started_on_year, DerivedField)
    assert started_on_year is not aliased
    assert started_on_year.name == "(year(startedOn))"
    assert aliased.name == "(year(startedOn))"
    assert started_on_year.alias is None
    assert aliased.alias == "startedOnYear"


def test_datefield_boolean_operation_returns_expected_boolean_expression():
    d = DateField("started_on_date")
    bool_expr = d > date(2023, 11, 1)
    # NB: Cannot use assert bool_expr.field == d because __eq__ is overloaded to
    # return a BooleanExpr type.
    assert isinstance(bool_expr.field, DateField)
    assert bool_expr.field.name == "started_on_date"
    assert bool_expr.op == "gt"
    assert isinstance(bool_expr.other, LiteralField)
    assert bool_expr.other.value == "2023-11-01"


def test_timefield_boolean_operation_returns_expected_boolean_expression():
    t = TimeField("started_at_time")
    bool_expr = t <= time(12, 8, 7)
    # NB: Cannot use assert bool_expr.field == t because __eq__ is overloaded to
    # return a BooleanExpr type.
    assert isinstance(bool_expr.field, TimeField)
    assert bool_expr.field.name == "started_at_time"
    assert bool_expr.op == "lte"
    assert isinstance(bool_expr.other, LiteralField)
    assert bool_expr.other.value == "12:08:07"


def test_datetimefield_boolean_operation_returns_expected_boolean_expression():
    dt = DateTimeField("started_at_time")
    bool_expr = dt <= datetime(2023, 11, 1, 12, 8, 7)
    # NB: Cannot use assert bool_expr.field == dt because __eq__ is overloaded
    # to return a BooleanExpr type.
    assert isinstance(bool_expr.field, DateTimeField)
    assert bool_expr.field.name == "started_at_time"
    assert bool_expr.op == "lte"
    assert isinstance(bool_expr.other, LiteralField)
    assert bool_expr.other.value == "2023-11-01T12:08:07"