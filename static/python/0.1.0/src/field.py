from typing import Any
from datetime import datetime, date

from fieldExpr import (
  AggregateFieldExpr,
  BooleanFieldExpr,
  BooleanOperator,
  FieldExpr,
  Operator,
  Scalar
)
    
class Field(FieldExpr):
    def __init__(self, name: str):
        super().__init__(name)
        self._val = None

    def operator(self) -> Operator:
        return 'ident'

    def operands(self) -> list:
        return [self.name]

    def as_(self, alias: str) -> FieldExpr:
        super().as_(alias)
        return self

    def asBooleanExpr(self, op: BooleanOperator, that: Scalar or list or FieldExpr) -> BooleanFieldExpr:
        that_ = that if isinstance(that, FieldExpr) else LiteralField(that)
        return BooleanFieldExpr(self, op, that_)

    def max(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, 'max')

    def min(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, 'min')

    def count(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, 'count')

    def countDistinct(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, 'countDistinct')

    def avg(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, 'avg')

    def avgDistinct(self) -> AggregateFieldExpr:
        return AggregateFieldExpr(self, 'avgDistinct')

    def __eq__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr: # ==
        return self.asBooleanExpr('eq', that)

    def __ne__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr: # !=
        return self.asBooleanExpr('neq', that)

    def __gt__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr: # >
        return self.asBooleanExpr('gt', that)

    def __ge__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr: # >=
        return self.asBooleanExpr('gte', that)

    def __lt__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr: # <
        return self.asBooleanExpr('lt', that)

    def __le__(self, that: Scalar or FieldExpr) -> BooleanFieldExpr: # <=
        return self.asBooleanExpr('lte', that)

    def _in(self, that: list) -> BooleanFieldExpr:
        return self.asBooleanExpr('in', that)

    def between(self, minVal: Scalar, maxVal: Scalar) -> BooleanFieldExpr:
        return self.gte(minVal).and_(self.lte(maxVal))

class LiteralField(Field):
    def __init__(self, value: Any):
        super().__init__(f"lit({value})")
        self.value = value

    def operator(self) -> Operator:
        return 'ident'

    def operands(self) -> list:
        if isinstance(self.value, list):
            return self.value
        return [self.value]

    def max(self):
        raise SyntaxError('Cannot call max on literal field')

    def min(self):
        raise SyntaxError('Cannot call min on literal field')

    def count(self):
        raise SyntaxError('Cannot call count on literal field')

    def countDistinct(self):
        raise SyntaxError('Cannot call countDistinct on literal field')

    def avg(self):
        raise SyntaxError('Cannot call avg on literal field')

    def avgDistinct(self):
        raise SyntaxError('Cannot call avgDistinct on literal field')

class StringField(Field):
    def __init__(self, name: str):
        super().__init__(name)

    def like(self, pattern: str) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, 'like', LiteralField(pattern))

class DerivedField(Field):
    def __init__(self, field: Field, op: str):
        super().__init__(f"({op}({field.name}))")
        self.op = op
        self.field = field

    def operator(self) -> str:
        return self.op

    def operands(self) -> list:
        return [self.field]

def toISODateString(d: datetime) -> str:
    return d.strftime('%Y-%m-%d')

class DateField(Field):
    def __init__(self, name: str):
        super().__init__(name)

    @property
    def month(self) -> DerivedField:
        return DerivedField[int, date](self, 'month')

    @property
    def day(self) -> DerivedField:
        return DerivedField(self, 'day')

    @property
    def year(self) -> DerivedField:
        return DerivedField(self, 'year')

    def before(self, d: date) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, 'lt', LiteralField(toISODateString(d)))

    def after(self, d: date) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, 'gt', LiteralField(toISODateString(d)))

    def __lt__(self, d: date) -> BooleanFieldExpr: # <
        return self.before(d)

    def __gt__(self, d: date) -> BooleanFieldExpr: # >
        return self.after(d)

    def inPast(self, olderThan: int, newerThan: int, granularity: str) -> BooleanFieldExpr:
        if olderThan > newerThan:
            print(f"inPast specified with olderThan({olderThan}) > newerThan({newerThan}), swapped arguments.")
            olderThan, newerThan = newerThan, olderThan
        return BooleanFieldExpr(self, 'inPast', LiteralField([olderThan, newerThan, granularity]))

class DateTimeField(DateField):
    def __init__(self, name: str):
        super().__init__(name)

    @property
    def hour(self) -> DerivedField:
        return DerivedField(self, 'hour')

    @property
    def minute(self) -> DerivedField:
        return DerivedField(self, 'minute')

    @property
    def second(self) -> DerivedField:
        return DerivedField(self, 'second')

    def before(self, d: datetime) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, 'lt', LiteralField(d.isoformat()))

    def after(self, d: datetime) -> BooleanFieldExpr:
        return BooleanFieldExpr(self, 'gt', LiteralField(d.isoformat()))

    def inPast(self, olderThan: int, newerThan: int, granularity: str) -> BooleanFieldExpr:
        if olderThan > newerThan:
            print(f"inPast specified with olderThan({olderThan}) > newerThan({newerThan}), swapped arguments.")
            olderThan, newerThan = newerThan, olderThan
        return BooleanFieldExpr(self, 'inPast', LiteralField([olderThan, newerThan, granularity]))
    