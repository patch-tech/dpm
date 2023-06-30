from typing import Any
from datetime import datetime, date
import logging

from .field_expr import (
    AggregateFieldExpr,
    BooleanFieldExpr,
    BooleanOperator,
    Expr,
    FieldExpr,
    Operator,
    Scalar,
    UnaryBooleanFieldExpr,
)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class Field(FieldExpr):
    def __init__(self, name: str):
        """A base class to represent a field in a `Table`. Identifies the underlying DB
        table column by its name."""
        super().__init__(name)
        self._val = None

    def operator(self) -> Operator:
        return "ident"

    def operands(self) -> list[Expr]:
        return [self.name]

    def with_alias(self, alias: str) -> FieldExpr:
        """
        Alias this field.

        Example:
            query = MyTable.select(field_with_long_name.with_alias('short_name'), price)
                           .order_by(['short_name', 'DESC'])
                           .limit(10)

        Args:
            alias: The alias for the field.

        Returns:
            A new Field object with the specified alias.
        """
        copy = Field(self.name)
        copy.alias = alias
        return copy

    def as_boolean_expr(
        self, op: BooleanOperator, that: Scalar or list or FieldExpr
    ) -> BooleanFieldExpr:
        that_ = that if isinstance(that, FieldExpr) else LiteralField(that)
        return BooleanFieldExpr(self, op, that_)

    def max(self) -> AggregateFieldExpr:
        """Returns a `max` aggregation applied on this field."""
        return AggregateFieldExpr(self, "max")

    def min(self) -> AggregateFieldExpr:
        """Returns a `min` aggregation applied on this field."""
        return AggregateFieldExpr(self, "min")

    def count(self) -> AggregateFieldExpr:
        """Returns a `count` aggregation applied on this field."""
        return AggregateFieldExpr(self, "count")

    def count_distinct(self) -> AggregateFieldExpr:
        """Returns a distinct `count` aggregation applied on this field."""
        return AggregateFieldExpr(self, "countDistinct")

    def avg(self) -> AggregateFieldExpr:
        """Returns an `average` aggregation applied on this field."""
        return AggregateFieldExpr(self, "avg")

    def avg_distinct(self) -> AggregateFieldExpr:
        """Returns a distinct `average` aggregation applied on this field."""
        return AggregateFieldExpr(self, "avgDistinct")

    def __eq__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # ==
        """Returns a boolean expression with an equality check."""
        return self.as_boolean_expr("eq", that)

    def __ne__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # !=
        """Returns a boolean expression with a not equal check."""
        return self.as_boolean_expr("neq", that)

    def __gt__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # >
        """Returns a boolean expression with greater than (>) check."""
        return self.as_boolean_expr("gt", that)

    def __ge__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # >=
        """Returns a boolean expression with greater than or equal (>=) check."""
        return self.as_boolean_expr("gte", that)

    def __lt__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # <
        """Returns a boolean expression with less than (<) check."""
        return self.as_boolean_expr("lt", that)

    def __le__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # <=
        """Returns a boolean expression with less than or equal (<=) check."""
        return self.as_boolean_expr("lte", that)

    def is_in(self, that: list) -> BooleanFieldExpr:
        """Returns a boolean expression with an array membership check. The field's
        value must exactly match at least one entry in `that` for the check to be
        true."""
        return self.as_boolean_expr("in", that)
    
    def is_null(self) -> UnaryBooleanFieldExpr:
        """Returns a boolean expression that checks if the field is null."""
        return UnaryBooleanFieldExpr(self, "isNull")
    
    def is_not_null(self) -> UnaryBooleanFieldExpr:
        """Returns a boolean expression that checks if the field is not null."""
        return UnaryBooleanFieldExpr(self, "isNotNull")
    
    def between(self, min_val: Scalar, max_val: Scalar) -> BooleanFieldExpr:
        """Returns a boolean expression that checks if the field's value is in between
        a range (inclusive of bounds)."""
        return (self >= min_val) & (self <= max_val)


class LiteralField(Field):
    def __init__(self, value: Any):
        """A literal field value. Used to represent constant RHS values in expressions."""
        super().__init__(f"lit({value})")
        self.value = value

    def operator(self) -> Operator:
        return "ident"

    def operands(self) -> list[Expr]:
        if isinstance(self.value, list):
            return self.value
        return [self.value]

    def max(self):
        raise SyntaxError("Cannot call max on literal field")

    def min(self):
        raise SyntaxError("Cannot call min on literal field")

    def count(self):
        raise SyntaxError("Cannot call count on literal field")

    def count_distinct(self):
        raise SyntaxError("Cannot call count_distinct on literal field")

    def avg(self):
        raise SyntaxError("Cannot call avg on literal field")

    def avg_distinct(self):
        raise SyntaxError("Cannot call avg_distinct on literal field")


class StringField(Field):
    def __init__(self, name: str):
        """A string field. Defines additional operators that are specific to strings."""
        super().__init__(name)

    def like(self, pattern: str) -> BooleanFieldExpr:
        """
        Returns a boolean expression for a string `like` check.
        See: https://en.wikibooks.org/wiki/Structured_Query_Language/Like_Predicate#LIKE

        Example:
            query = MyTable.select(name, price)
                           .filter(name.like('%shirt%'))
                           .limit(10)

        Args:
            pattern: The like pattern with wildcards: % (one or more) and _ (exactly one).

        Returns:
            A BooleanFieldExpr object representing the boolean expression for the `like` check.
        """
        return BooleanFieldExpr(self, "like", LiteralField(pattern))


class DerivedField(Field):
    def __init__(self, field: Field, op: str):
        """
        A derived field obtained by applying a projection operator.

        Example:
            start_date_time = DateTimeField('started_at')
            start_year = DerivedField(start_date_time, 'year')

        See Also:
            - `year` method of `DateField`
            - `month` method of `DateField`
            - `day` method of `DateField`
            for getters that return derived fields.
        """
        super().__init__(f"({op}({field.name}))")
        self.op = op
        self.field = field

    def operator(self) -> Operator:
        return self.op

    def operands(self) -> list[Expr]:
        return [self.field]

    def with_alias(self, alias: str) -> "DerivedField":
        copy =  DerivedField(self.field, self.op)
        copy.alias = alias
        return copy


def to_iso_datestring(d: datetime) -> str:
    """
    Returns the date formatted as YYYY-MM-DD.

    See RFC3339 and https://www.w3.org/TR/NOTE-datetime

    Args:
        d: The date to be formatted.

    Returns:
        The date formatted as YYYY-MM-DD.
    """
    return d.strftime("%Y-%m-%d")


class DateField(Field):
    def __init__(self, name: str):
        super().__init__(name)

    @property
    def month(self) -> DerivedField:
        """Projects the date to its month."""
        return DerivedField[int, date](self, "month")

    @property
    def day(self) -> DerivedField:
        """Projects the date to its day."""
        return DerivedField(self, "day")

    @property
    def year(self) -> DerivedField:
        """Projects the date to its year."""
        return DerivedField(self, "year")

    def before(self, d: date) -> BooleanFieldExpr:
        """
        Returns a boolean expression that checks if this date is before `d`.

        Args:
            d: The date to compare against.

        Returns:
            A BooleanFieldExpr representing the boolean expression.
        """
        return BooleanFieldExpr(self, "lt", LiteralField(to_iso_datestring(d)))

    def after(self, d: date) -> BooleanFieldExpr:
        """
        Returns a boolean expression that checks if this date is after `d`.

        Args:
            d: The date to compare against.

        Returns:
            A BooleanFieldExpr representing the boolean expression.
        """
        return BooleanFieldExpr(self, "gt", LiteralField(to_iso_datestring(d)))

    # TODO(PAT-3290): Implement ==, !=, <=, >=

    def __lt__(self, d: date) -> BooleanFieldExpr:  # <
        """
        Returns a boolean expression that checks if this date is less than (<) `d`.

        Args:
            d: The date to compare against.

        Returns:
            A BooleanFieldExpr representing the boolean expression.
        """
        return self.before(d)

    def __gt__(self, d: date) -> BooleanFieldExpr:  # >
        """
        Returns a boolean expression that checks if this date is greater than (>) `d`.

        Args:
            d: The date to compare against.

        Returns:
            A BooleanFieldExpr representing the boolean expression.
        """
        return self.after(d)

    def in_past(
        self, older_than: int, newer_than: int, granularity: str
    ) -> BooleanFieldExpr:
        """
        Returns a boolean expression that performs a relative range check of this date.
        The range is specified by its two bounds and a granularity.
        E.g., the filter expression below checks if the value of `start_date` lies
        in the past 2 to 3 weeks.

        Example:
            query = MyTable.select(start_date, name).filter(start_date.in_past(2, 3, 'weeks'))

        Args:
            older_than: The number of units older than the current date.
            newer_than: The number of units newer than the current date.
            granularity: The granularity of the range (e.g., 'weeks', 'days', 'months').

        Returns:
            A BooleanFieldExpr representing the boolean expression.
        """
        if older_than > newer_than:
            logging.warn(
                f"inPast specified with older_than({older_than}) > newer_than({newer_than}), swapped arguments."
            )
            older_than, newer_than = newer_than, older_than
        return BooleanFieldExpr(
            self, "inPast", LiteralField([older_than, newer_than, granularity])
        )


class DateTimeField(DateField):
    def __init__(self, name: str):
        super().__init__(name)

    @property
    def hour(self) -> DerivedField:
        """Projects the time to its hour."""
        return DerivedField(self, "hour")

    @property
    def minute(self) -> DerivedField:
        """Projects the time to its minute."""
        return DerivedField(self, "minute")

    @property
    def second(self) -> DerivedField:
        """Projects the time to its second."""
        return DerivedField(self, "second")

    def before(self, d: datetime) -> BooleanFieldExpr:
        """
        Returns a boolean expression that checks if this datetime is before `d`.

        Args:
            d: The datetime to compare against.

        Returns:
            A BooleanFieldExpr representing the boolean expression.
        """
        return BooleanFieldExpr(self, "lt", LiteralField(d.isoformat()))

    def after(self, d: datetime) -> BooleanFieldExpr:
        """
        Returns a boolean expression that checks if this datetime is after `d`.

        Args:
            d: The datetime to compare against.

        Returns:
            A BooleanFieldExpr representing the boolean expression.
        """
        return BooleanFieldExpr(self, "gt", LiteralField(d.isoformat()))

    def in_past(
        self, older_than: int, newer_than: int, granularity: str
    ) -> BooleanFieldExpr:
        """
        Returns a boolean expression that performs a relative range check of this datetime.
        The range is specified by its two bounds and a granularity.
        E.g., the filter expression below checks if the value of `start_date_time` lies
        in the past 2 to 3 hours.

        Example:
            query = MyTable.select(start_date_time, name).filter(start_date_time.in_past(2, 3, 'hours'))

        Parameters:
            older_than (int): The number of units older than the current datetime.
            newer_than (int): The number of units newer than the current datetime.
            granularity (str): The granularity of the range (e.g., 'hours', 'days', 'months').

        Returns:
            BooleanFieldExpr: A BooleanFieldExpr representing the boolean expression.
        """
        if older_than > newer_than:
            logger.warn(
                f"inPast specified with older_than({older_than}) > newer_than({newer_than}), swapped arguments."
            )
            older_than, newer_than = newer_than, older_than
        return BooleanFieldExpr(
            self, "inPast", LiteralField([older_than, newer_than, granularity])
        )
