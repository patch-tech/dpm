from typing import Any, Callable, Union, List, Optional, Literal, TypeVar

Scalar = Union[str, int, float, bool]
UnaryOperator = Union[
    Literal["isNull"],
    Literal["isNotNull"]
]
BooleanOperator = Union[
    Literal["eq"],
    Literal["neq"],
    Literal["gt"],
    Literal["gte"],
    Literal["lt"],
    Literal["lte"],
    Literal["and"],
    Literal["or"],
]
ArithmeticOperator = Union[Literal["+"], Literal["-"], Literal["*"], Literal["/"]]
AggregateOperator = Union[
    Literal["min"], Literal["max"], Literal["count"], Literal["countDistinct"]
]
DateOperator = Union[Literal["years"], Literal["months"], Literal["days"]]
TimeOperator = Union[
    Literal["hour"], Literal["minute"], Literal["second"], Literal["millisecond"]
]
ProjectionOperator = Union[DateOperator, TimeOperator]
DateGranularity = Union[
    Literal["years"], Literal["months"], Literal["weeks"], Literal["days"]
]
TimeGranularity = Union[
    Literal["hours"], Literal["minutes"], Literal["seconds"], Literal["milliseconds"]
]
DateTimeGranularity = Union[DateGranularity, TimeGranularity]
Operator = Union[UnaryOperator, BooleanOperator, ArithmeticOperator, AggregateOperator]


class FieldExpr:

    name: str
    alias: Optional[str] = None

    def __init__(self, name: str, alias: Optional[str] = None):
        self.name = name
        self.alias = alias

    def to_string(self) -> str:
        return self.name

    def operator(self) -> Operator:
        pass

    def operands(self) -> List[Union['FieldExpr', Scalar]]:
        return []

Expr = Union[Scalar, FieldExpr]


class BooleanFieldExpr(FieldExpr):
    def __init__(
        self,
        field: FieldExpr,
        op: BooleanOperator,
        other: FieldExpr,
        alias: Optional[str] = None,
    ):
        super().__init__(field, alias)
        self.field = field
        self.op = op
        self.other = other

    def operator(self) -> Operator:
        return self.op

    def operands(self) -> List[Expr]:
        return [self.field, self.other]

    def __and__(self, that: FieldExpr) -> "BooleanFieldExpr":  # &
        return BooleanFieldExpr(self, "and", that)

    def __or__(self, that: FieldExpr) -> "BooleanFieldExpr":  # |
        return BooleanFieldExpr(self, "or", that)
    
    
class UnaryBooleanFieldExpr(FieldExpr):
    def __init__(self, field: FieldExpr, op: UnaryOperator) -> None:
        super().__init__(field, op)
        self.field = field
        self.op = op

    def __and__(self, that: FieldExpr) -> "BooleanFieldExpr":  # &
        return BooleanFieldExpr(self, "and", that)

    def __or__(self, that: FieldExpr) -> "BooleanFieldExpr":  # |
        return BooleanFieldExpr(self, "or", that)
    
    def operator(self) -> Operator:
        return self.op

    def operands(self) -> List[Expr]:
        return [self.field]



class AggregateFieldExpr(FieldExpr):
    def __init__(self, field: FieldExpr, op: AggregateOperator) -> None:
        super().__init__(f"({op}({field.name}))")
        self.field = field
        self.op = op

    def operator(self) -> Operator:
        return self.op

    def operands(self) -> List[Expr]:
        return [self.field]

    def with_alias(self, alias: str) -> "AggregateFieldExpr":
        copy =  AggregateFieldExpr(self.field, self.op)
        copy.alias = alias
        return copy


# TODO(PAT-3177): Define ArithmeticFieldExpr?
