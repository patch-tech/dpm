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
    # User-specified alias for expression. Can be used in a `select` and then in
    # a subsequent `order_by`.
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
        """
        A binary boolean expression. Can be combined with other boolean expressions
        using `and`, `or` methods.
        """
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
        """
        A unary boolean expression.

        Args:
            field: The field expression to perform the unary operation on.
            op: The unary operator to apply to the field expression.
        """
        super().__init__(("(" + op + "(" + field.name + "))"))
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
        """
        A field expression to represent an aggregation applied on a field expression.

        Example usage:
        >>> price = Field('price')
        >>> total_price = AggregateFieldExpr(price, 'sum')

        Attributes:
            field: The field expression to apply the aggregation on.
            op: The aggregate operator to use for the aggregation.
        """
        super().__init__(f"({op}({field.name}))")
        self.field = field
        self.op = op

    def operator(self) -> Operator:
        return self.op

    def operands(self) -> List[Expr]:
        return [self.field]

    def with_alias(self, alias: str) -> "AggregateFieldExpr":
        """
        Alias this expression. This method is useful when the aggregate expression
        is defined in a `select` and must be referred to in a subsequent `order_by`.

        Example usage:
        >>> query = MyTable.select(name, price.sum().with_alias('totalPrice'))
                           .order_by(['totalPrice', 'DESC'])
                           .limit(10)

        Args:
            alias: The alias to assign to the aggregate expression.

        Returns:
            An `AggregateFieldExpr` object with the specified alias.
        """
        copy =  AggregateFieldExpr(self.field, self.op)
        copy.alias = alias
        return copy


# TODO(PAT-3177): Define ArithmeticFieldExpr?
