from typing import Any, Callable, Union, List, Optional, Literal, TypeVar

Scalar = Union[str, int, float, bool]
UnaryOperator = Union[Literal['not'], Literal['-'], Literal['gt'], Literal['gte'], Literal['lt'], Literal['lte']]
BooleanOperator = Union[Literal["eq"], Literal["neq"], Literal["gt"], Literal["gte"], Literal["lt"], Literal["lte"], Literal["and"], Literal["or"]]
ArithmeticOperator = Union[Literal["+"], Literal["-"], Literal["*"], Literal["/"]]
AggregateOperator = Union[Literal["min"], Literal["max"], Literal["count"], Literal["countDistinct"]]
DateOperator = Union[Literal["years"], Literal["months"], Literal["days"]]
TimeOperator = Union[Literal["hour"], Literal["minute"], Literal["second"], Literal["millisecond"]]
ProjectionOperator = Union[DateOperator, TimeOperator]
DateGranularity = Union[Literal["years"], Literal["months"], Literal["weeks"],Literal["days"]]
TimeGranularity = Union[Literal["hours"], Literal["minutes"], Literal["seconds"], Literal["milliseconds"]]
DateTimeGranularity = Union[DateGranularity, TimeGranularity]
Operator = Union[UnaryOperator, BooleanOperator, ArithmeticOperator, AggregateOperator]

class FieldExpr:
    name: str
    alias: Optional[str] = None

    def __init__(self, name: str, alias: Optional[str] = None):
        self.name = name
        self.alias = alias

    def toString(self) -> str:
        return self.name

    def as_(self, alias: str) -> 'FieldExpr':
        return FieldExpr(self, alias)

    def operator(self) -> Callable:
        pass

    def operands(self) -> List[Union[str, int, float, bool]]:
        return []


class UnaryFieldExpr(FieldExpr):
    def __init__(self, field: FieldExpr, op: UnaryOperator, alias: Optional[str] = None):
        super().__init__(("(" + op + "(" + field.name + "))"))
        self.field = field
        self.op = op
        self.alias = alias

    def operator(self) -> Callable:
        return self.op

    def operands(self) -> List[Union[str, int, float, bool]]:
        return [self.field]

    def not_(self) -> 'UnaryFieldExpr':
        return UnaryFieldExpr(self, 'not')

    def minus(self) -> 'UnaryFieldExpr':
        return UnaryFieldExpr(self, 'minus')

    def plus(self) -> 'UnaryFieldExpr':
        return UnaryFieldExpr(self, 'plus')

    def lt(self) -> 'UnaryFieldExpr':
        return UnaryFieldExpr(self, 'lt')

    def gt(self) -> 'UnaryFieldExpr':
        return UnaryFieldExpr(self, 'gt')

    def gte(self) -> 'UnaryFieldExpr':
        return UnaryFieldExpr(self, 'gte')


class BooleanFieldExpr(FieldExpr):
    def __init__(self, field: FieldExpr, op: BooleanOperator, other: FieldExpr, alias: Optional[str] = None):
        super().__init__(field, alias)
        self.op = op
        self.other = other

    def operator(self) -> Callable:
        return self.op

    def operands(self) -> List[Union[str, int, float, bool]]:
        return [self.field, self.other]

    def and_(self) -> 'BooleanFieldExpr':
        return UnaryFieldExpr(self, 'and')

    def or_(self) -> 'BooleanFieldExpr':
        return UnaryFieldExpr(self, 'or')

    def not_(self) -> 'BooleanFieldExpr':
        return UnaryFieldExpr(self, 'not')

class AggregateFieldExpr(FieldExpr):
    def __init__(self, field: FieldExpr, op: AggregateOperator) -> None:
        super().__init__(f"({op}({field.name}))")
        self.field = field
        self.op = op

    def operator(self) -> Operator:
        return self.op

    def operands(self) -> List[FieldExpr]:
        return [self.field]

    def as_(self, alias: str) -> "AggregateFieldExpr":
        super().as_(alias)
        return self
    
Expr = Union[Scalar, UnaryFieldExpr, BooleanFieldExpr]
