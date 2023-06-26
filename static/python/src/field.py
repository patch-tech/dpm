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
        super().__init__(name)
        self._val = None

    def operator(self) -> Operator:
        return "ident"

    def operands(self) -> list[Expr]:
        return [self.name]

    def with_alias(self, alias: str) -> FieldExpr:
        copy = Field(self.name)
        copy.alias = alias
        return copy

    def as_boolean_expr(
        self, op: BooleanOperator, that: Scalar or list or FieldExpr
    ) -> BooleanFieldExpr:
        that_ = that if isinstance(that, FieldExpr) else LiteralField(that)
        return BooleanFieldExpr(self, op, that_)

    def max(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, "max")

    def min(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, "min")

    def count(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, "count")

    def count_distinct(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, "countDistinct")

    def avg(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, "avg")

    def avg_distinct(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, "avgDistinct")

    def __eq__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # ==
        return self.as_boolean_expr("eq", that)

    def __ne__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # !=
        return self.as_boolean_expr("neq", that)

    def __gt__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # >
        return self.as_boolean_expr("gt", that)

    def __ge__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # >=
        return self.as_boolean_expr("gte", that)

    def __lt__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # <
        return self.as_boolean_expr("lt", that)

    def __le__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr:  # <=
        return self.as_boolean_expr("lte", that)

    def is_in(self, that: list) -> BooleanFieldExpr:
        return self.as_boolean_expr("in", that)
    
    def is_null(self) -> UnaryBooleanFieldExpr:
        return UnaryBooleanFieldExpr(self, "isNull")
    
    def is_not_null(self) -> UnaryBooleanFieldExpr:
        return UnaryBooleanFieldExpr(self, "isNotNull")
    
    def between(self, min_val: Scalar, max_val: Scalar) -> BooleanFieldExpr:
        return (self >= min_val) & (self <= max_val)


class LiteralField(Field):
    def __init__(self, value: Any):
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
        super().__init__(name)

    def like(self, pattern: str) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, "like", LiteralField(pattern))


class DerivedField(Field):
    def __init__(self, field: Field, op: str):
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
    return d.strftime("%Y-%m-%d")


class DateField(Field):
    def __init__(self, name: str):
        super().__init__(name)

    @property
    def month(self) -> DerivedField:
        return DerivedField[int, date](self, "month")

    @property
    def day(self) -> DerivedField:
        return DerivedField(self, "day")

    @property
    def year(self) -> DerivedField:
        return DerivedField(self, "year")

    def before(self, d: date) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, "lt", LiteralField(to_iso_datestring(d)))

    def after(self, d: date) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, "gt", LiteralField(to_iso_datestring(d)))

    # TODO(PAT-3290): Implement ==, !=, <=, >=

    def __lt__(self, d: date) -> BooleanFieldExpr:  # <
        return self.before(d)

    def __gt__(self, d: date) -> BooleanFieldExpr:  # >
        return self.after(d)

    def in_past(
        self, older_than: int, newer_than: int, granularity: str
    ) -> BooleanFieldExpr:
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
        return DerivedField(self, "hour")

    @property
    def minute(self) -> DerivedField:
        return DerivedField(self, "minute")

    @property
    def second(self) -> DerivedField:
        return DerivedField(self, "second")

    def before(self, d: datetime) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, "lt", LiteralField(d.isoformat()))

    def after(self, d: datetime) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, "gt", LiteralField(d.isoformat()))

    def in_past(
        self, older_than: int, newer_than: int, granularity: str
    ) -> BooleanFieldExpr:
        if older_than > newer_than:
            logger.warn(
                f"inPast specified with older_than({older_than}) > newer_than({newer_than}), swapped arguments."
            )
            older_than, newer_than = newer_than, older_than
        return BooleanFieldExpr(
            self, "inPast", LiteralField([older_than, newer_than, granularity])
        )
