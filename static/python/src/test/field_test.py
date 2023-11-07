from datetime import date, time, datetime

import pytest

from ..field import (
    add_duration,
    ArrayField,
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
    bool_expr = dt <= datetime.fromisoformat("2023-11-01T12:08:07-07:53")
    # NB: Cannot use assert bool_expr.field == dt because __eq__ is overloaded
    # to return a BooleanExpr type.
    assert isinstance(bool_expr.field, DateTimeField)
    assert bool_expr.field.name == "started_at_time"
    assert bool_expr.op == "lte"
    assert isinstance(bool_expr.other, LiteralField)
    # RHS time is stored in UTC.
    assert bool_expr.other.value == "2023-11-01T20:01:07.000000Z"


def test_add_duration_returns_expected_results():
    # date.
    assert add_duration(date(year=2023, month=10, day=12), -1, "years") == date(
        year=2022, month=10, day=12
    )
    assert add_duration(date(year=2023, month=2, day=15), 13, "days") == date(
        year=2023, month=2, day=28
    )
    assert add_duration(date(year=2023, month=2, day=15), 2, "weeks") == date(
        year=2023, month=3, day=1
    )

    # time.
    # Clamps to zero.
    assert add_duration(time(hour=15, minute=2, second=45), -16, "hours") == time(
        hour=0
    )
    assert add_duration(time(hour=15, minute=2, second=45), -1024, "hours") == time(
        hour=0
    )
    # Clamps to last time of day.
    assert add_duration(time(hour=15, minute=2, second=45), 9, "hours") == time(
        hour=23, minute=59, second=59, microsecond=999999
    )
    assert add_duration(time(hour=15, minute=2, second=45), 9000, "hours") == time(
        hour=23, minute=59, second=59, microsecond=999999
    )

    assert add_duration(time(hour=15, minute=2, second=45), -12, "minutes") == time(
        hour=14, minute=50, second=45
    )
    # Milliseconds.
    assert add_duration(
        time(hour=15, minute=2, second=45), 15000, "milliseconds"
    ) == time(hour=15, minute=3, second=0)

    # datetime.
    dt = datetime(year=2023, month=2, day=15, hour=15, minute=2, second=45)
    assert add_duration(dt, -1, "years") == datetime(
        year=2022, month=2, day=15, hour=15, minute=2, second=45
    )
    assert add_duration(dt, 13, "days") == datetime(
        year=2023, month=2, day=28, hour=15, minute=2, second=45
    )
    assert add_duration(dt, 2, "weeks") == datetime(
        year=2023, month=3, day=1, hour=15, minute=2, second=45
    )
    # Milliseconds.
    assert add_duration(dt, 15123, "milliseconds") == datetime(
        year=2023, month=2, day=15, hour=15, minute=3, second=0, microsecond=123000
    )


def test_datefield_in_past_returns_expected_boolean_expression():
    d = DateField("started_on_date")
    bool_expr = d.in_past(1, 2, "weeks")
    # NB: Cannot use assert bool_expr.field == d because __eq__ is overloaded to
    # return a BooleanExpr type.
    assert bool_expr.operator() == "and"
    (operand1, operand2) = bool_expr.operands()

    assert operand1.operator() == "gte"
    lhs = operand1.operands()[0]
    assert isinstance(lhs, DateField)
    assert lhs.name == "started_on_date"

    assert operand2.operator() == "lte"
    lhs = operand2.operands()[0]
    assert isinstance(lhs, DateField)
    assert lhs.name == "started_on_date"

    # NB: Testing the exact RHS values is flaky because it depends on the
    # current time. The tests for `add_duration` should exercise the correctness
    # of the computed time bounds.
    lower = operand1.operands()[1]
    assert isinstance(lower, LiteralField)
    upper = operand2.operands()[1]
    assert isinstance(upper, LiteralField)
    try:
        assert date.fromisoformat(lower.value) <= date.fromisoformat(upper.value)
    except ValueError as verr:
        pytest.fail(f"DateField in_past produced invalid ranges '{verr}'")


def test_timefield_in_past_returns_expected_boolean_expression():
    t = TimeField("started_at_time")
    bool_expr = t.in_past(1, 2, "hours")
    # NB: Cannot use assert bool_expr.field == d because __eq__ is overloaded to
    # return a BooleanExpr type.
    assert bool_expr.operator() == "and"
    (operand1, operand2) = bool_expr.operands()

    assert operand1.operator() == "gte"
    lhs = operand1.operands()[0]
    assert isinstance(lhs, TimeField)
    assert lhs.name == "started_at_time"

    assert operand2.operator() == "lte"
    lhs = operand2.operands()[0]
    assert isinstance(lhs, TimeField)
    assert lhs.name == "started_at_time"

    # NB: Testing the exact RHS values is flaky because it depends on the
    # current time. The tests for `add_duration` should exercise the correctness
    # of the computed time bounds.
    lower = operand1.operands()[1]
    assert isinstance(lower, LiteralField)
    upper = operand2.operands()[1]
    assert isinstance(upper, LiteralField)
    try:
        assert time.fromisoformat(lower.value) <= time.fromisoformat(upper.value)
    except ValueError as verr:
        pytest.fail(f"TimeField in_past produced invalid ranges '{verr}'")


def test_datetimefield_in_past_returns_expected_boolean_expression():
    dt = DateTimeField("started_at_time")
    bool_expr = dt.in_past(1, 2, "years")
    # NB: Cannot use assert bool_expr.field == d because __eq__ is overloaded to
    # return a BooleanExpr type.
    assert bool_expr.operator() == "and"
    (operand1, operand2) = bool_expr.operands()

    assert operand1.operator() == "gte"
    lhs = operand1.operands()[0]
    assert isinstance(lhs, DateTimeField)
    assert lhs.name == "started_at_time"

    assert operand2.operator() == "lte"
    lhs = operand2.operands()[0]
    assert isinstance(lhs, DateTimeField)
    assert lhs.name == "started_at_time"

    # NB: Testing the exact RHS values is flaky because it depends on the
    # current time. The tests for `add_duration` should exercise the correctness
    # of the computed time bounds.
    lower = operand1.operands()[1]
    assert isinstance(lower, LiteralField)
    upper = operand2.operands()[1]
    assert isinstance(upper, LiteralField)
    fmt = "%Y-%m-%dT%H:%M:%S.%fZ"
    try:
        assert datetime.strptime(lower.value, fmt) <= datetime.strptime(
            upper.value, fmt
        )
    except ValueError as verr:
        pytest.fail(f"DateTimeField in_past produced invalid ranges '{verr}'")


def test_arrayfield_has_any_returns_expected_boolean_expression():
    tag_names = ArrayField("tag_names")
    has_any_tags = tag_names.has_any(["foo", "bar"])

    assert has_any_tags.operator() == "hasAny"

    (operand1, operand2) = has_any_tags.operands()
    assert isinstance(operand1, ArrayField)
    assert operand1.name == "tag_names"
    assert isinstance(operand2, LiteralField)
    assert operand2.value == ["foo", "bar"]


def test_arrayfield_has_all_returns_expected_boolean_expression():
    tag_names = ArrayField("tag_names")
    has_all_tags = tag_names.has_all(["foo", "bar"])

    assert has_all_tags.operator() == "hasAll"

    (operand1, operand2) = has_all_tags.operands()
    assert isinstance(operand1, ArrayField)
    assert operand1.name == "tag_names"
    assert isinstance(operand2, LiteralField)
    assert operand2.value == ["foo", "bar"]
