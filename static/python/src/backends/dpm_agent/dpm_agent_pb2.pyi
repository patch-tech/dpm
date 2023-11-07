from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Query(_message.Message):
    __slots__ = ["id", "selectFrom", "select", "filter", "groupBy", "orderBy", "limit", "dryRun", "clientVersion", "type", "joins", "tableAlias"]
    class Type(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
        DATA: _ClassVar[Query.Type]
        INTROSPECTION: _ClassVar[Query.Type]
    DATA: Query.Type
    INTROSPECTION: Query.Type
    class Id(_message.Message):
        __slots__ = ["packageId", "sourceId"]
        PACKAGEID_FIELD_NUMBER: _ClassVar[int]
        SOURCEID_FIELD_NUMBER: _ClassVar[int]
        packageId: str
        sourceId: str
        def __init__(self, packageId: _Optional[str] = ..., sourceId: _Optional[str] = ...) -> None: ...
    class SelectExpression(_message.Message):
        __slots__ = ["argument", "alias"]
        ARGUMENT_FIELD_NUMBER: _ClassVar[int]
        ALIAS_FIELD_NUMBER: _ClassVar[int]
        argument: Query.Expression
        alias: str
        def __init__(self, argument: _Optional[_Union[Query.Expression, _Mapping]] = ..., alias: _Optional[str] = ...) -> None: ...
    class Expression(_message.Message):
        __slots__ = ["field", "literal", "derived", "aggregate", "condition"]
        FIELD_FIELD_NUMBER: _ClassVar[int]
        LITERAL_FIELD_NUMBER: _ClassVar[int]
        DERIVED_FIELD_NUMBER: _ClassVar[int]
        AGGREGATE_FIELD_NUMBER: _ClassVar[int]
        CONDITION_FIELD_NUMBER: _ClassVar[int]
        field: Query.FieldReference
        literal: Query.Literal
        derived: Query.DerivedExpression
        aggregate: Query.AggregateExpression
        condition: Query.BooleanExpression
        def __init__(self, field: _Optional[_Union[Query.FieldReference, _Mapping]] = ..., literal: _Optional[_Union[Query.Literal, _Mapping]] = ..., derived: _Optional[_Union[Query.DerivedExpression, _Mapping]] = ..., aggregate: _Optional[_Union[Query.AggregateExpression, _Mapping]] = ..., condition: _Optional[_Union[Query.BooleanExpression, _Mapping]] = ...) -> None: ...
    class FieldReference(_message.Message):
        __slots__ = ["fieldName", "tableName"]
        FIELDNAME_FIELD_NUMBER: _ClassVar[int]
        TABLENAME_FIELD_NUMBER: _ClassVar[int]
        fieldName: str
        tableName: str
        def __init__(self, fieldName: _Optional[str] = ..., tableName: _Optional[str] = ...) -> None: ...
    class Literal(_message.Message):
        __slots__ = ["string", "boolean", "ui32", "ui64", "i32", "i64", "f32", "f64", "timestamp", "list"]
        class List(_message.Message):
            __slots__ = ["values"]
            VALUES_FIELD_NUMBER: _ClassVar[int]
            values: _containers.RepeatedCompositeFieldContainer[Query.Literal]
            def __init__(self, values: _Optional[_Iterable[_Union[Query.Literal, _Mapping]]] = ...) -> None: ...
        STRING_FIELD_NUMBER: _ClassVar[int]
        BOOLEAN_FIELD_NUMBER: _ClassVar[int]
        UI32_FIELD_NUMBER: _ClassVar[int]
        UI64_FIELD_NUMBER: _ClassVar[int]
        I32_FIELD_NUMBER: _ClassVar[int]
        I64_FIELD_NUMBER: _ClassVar[int]
        F32_FIELD_NUMBER: _ClassVar[int]
        F64_FIELD_NUMBER: _ClassVar[int]
        TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
        LIST_FIELD_NUMBER: _ClassVar[int]
        string: str
        boolean: bool
        ui32: int
        ui64: int
        i32: int
        i64: int
        f32: float
        f64: float
        timestamp: int
        list: Query.Literal.List
        def __init__(self, string: _Optional[str] = ..., boolean: bool = ..., ui32: _Optional[int] = ..., ui64: _Optional[int] = ..., i32: _Optional[int] = ..., i64: _Optional[int] = ..., f32: _Optional[float] = ..., f64: _Optional[float] = ..., timestamp: _Optional[int] = ..., list: _Optional[_Union[Query.Literal.List, _Mapping]] = ...) -> None: ...
    class DerivedExpression(_message.Message):
        __slots__ = ["op", "argument"]
        class ProjectionOperator(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
            __slots__ = []
            YEAR: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            MONTH: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            DAY: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            HOUR: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            MINUTE: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            SECOND: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            MILLISECOND: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            DATE: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            TIME: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            WEEK: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            DAY_OF_WEEK: _ClassVar[Query.DerivedExpression.ProjectionOperator]
            DATE_OF_WEEK: _ClassVar[Query.DerivedExpression.ProjectionOperator]
        YEAR: Query.DerivedExpression.ProjectionOperator
        MONTH: Query.DerivedExpression.ProjectionOperator
        DAY: Query.DerivedExpression.ProjectionOperator
        HOUR: Query.DerivedExpression.ProjectionOperator
        MINUTE: Query.DerivedExpression.ProjectionOperator
        SECOND: Query.DerivedExpression.ProjectionOperator
        MILLISECOND: Query.DerivedExpression.ProjectionOperator
        DATE: Query.DerivedExpression.ProjectionOperator
        TIME: Query.DerivedExpression.ProjectionOperator
        WEEK: Query.DerivedExpression.ProjectionOperator
        DAY_OF_WEEK: Query.DerivedExpression.ProjectionOperator
        DATE_OF_WEEK: Query.DerivedExpression.ProjectionOperator
        OP_FIELD_NUMBER: _ClassVar[int]
        ARGUMENT_FIELD_NUMBER: _ClassVar[int]
        op: Query.DerivedExpression.ProjectionOperator
        argument: Query.Expression
        def __init__(self, op: _Optional[_Union[Query.DerivedExpression.ProjectionOperator, str]] = ..., argument: _Optional[_Union[Query.Expression, _Mapping]] = ...) -> None: ...
    class AggregateExpression(_message.Message):
        __slots__ = ["op", "argument"]
        class AggregateOperator(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
            __slots__ = []
            MIN: _ClassVar[Query.AggregateExpression.AggregateOperator]
            MAX: _ClassVar[Query.AggregateExpression.AggregateOperator]
            MEAN: _ClassVar[Query.AggregateExpression.AggregateOperator]
            MEDIAN: _ClassVar[Query.AggregateExpression.AggregateOperator]
            COUNT: _ClassVar[Query.AggregateExpression.AggregateOperator]
            COUNT_DISTINCT: _ClassVar[Query.AggregateExpression.AggregateOperator]
            SUM: _ClassVar[Query.AggregateExpression.AggregateOperator]
            MEAN_DISTINCT: _ClassVar[Query.AggregateExpression.AggregateOperator]
        MIN: Query.AggregateExpression.AggregateOperator
        MAX: Query.AggregateExpression.AggregateOperator
        MEAN: Query.AggregateExpression.AggregateOperator
        MEDIAN: Query.AggregateExpression.AggregateOperator
        COUNT: Query.AggregateExpression.AggregateOperator
        COUNT_DISTINCT: Query.AggregateExpression.AggregateOperator
        SUM: Query.AggregateExpression.AggregateOperator
        MEAN_DISTINCT: Query.AggregateExpression.AggregateOperator
        OP_FIELD_NUMBER: _ClassVar[int]
        ARGUMENT_FIELD_NUMBER: _ClassVar[int]
        op: Query.AggregateExpression.AggregateOperator
        argument: Query.Expression
        def __init__(self, op: _Optional[_Union[Query.AggregateExpression.AggregateOperator, str]] = ..., argument: _Optional[_Union[Query.Expression, _Mapping]] = ...) -> None: ...
    class BooleanExpression(_message.Message):
        __slots__ = ["op", "arguments"]
        class BooleanOperator(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
            __slots__ = []
            AND: _ClassVar[Query.BooleanExpression.BooleanOperator]
            OR: _ClassVar[Query.BooleanExpression.BooleanOperator]
            EQ: _ClassVar[Query.BooleanExpression.BooleanOperator]
            NEQ: _ClassVar[Query.BooleanExpression.BooleanOperator]
            LT: _ClassVar[Query.BooleanExpression.BooleanOperator]
            LTE: _ClassVar[Query.BooleanExpression.BooleanOperator]
            GT: _ClassVar[Query.BooleanExpression.BooleanOperator]
            GTE: _ClassVar[Query.BooleanExpression.BooleanOperator]
            LIKE: _ClassVar[Query.BooleanExpression.BooleanOperator]
            BETWEEN: _ClassVar[Query.BooleanExpression.BooleanOperator]
            IN: _ClassVar[Query.BooleanExpression.BooleanOperator]
            IS_NULL: _ClassVar[Query.BooleanExpression.BooleanOperator]
            IS_NOT_NULL: _ClassVar[Query.BooleanExpression.BooleanOperator]
            HAS_ANY: _ClassVar[Query.BooleanExpression.BooleanOperator]
            HAS_ALL: _ClassVar[Query.BooleanExpression.BooleanOperator]
        AND: Query.BooleanExpression.BooleanOperator
        OR: Query.BooleanExpression.BooleanOperator
        EQ: Query.BooleanExpression.BooleanOperator
        NEQ: Query.BooleanExpression.BooleanOperator
        LT: Query.BooleanExpression.BooleanOperator
        LTE: Query.BooleanExpression.BooleanOperator
        GT: Query.BooleanExpression.BooleanOperator
        GTE: Query.BooleanExpression.BooleanOperator
        LIKE: Query.BooleanExpression.BooleanOperator
        BETWEEN: Query.BooleanExpression.BooleanOperator
        IN: Query.BooleanExpression.BooleanOperator
        IS_NULL: Query.BooleanExpression.BooleanOperator
        IS_NOT_NULL: Query.BooleanExpression.BooleanOperator
        HAS_ANY: Query.BooleanExpression.BooleanOperator
        HAS_ALL: Query.BooleanExpression.BooleanOperator
        OP_FIELD_NUMBER: _ClassVar[int]
        ARGUMENTS_FIELD_NUMBER: _ClassVar[int]
        op: Query.BooleanExpression.BooleanOperator
        arguments: _containers.RepeatedCompositeFieldContainer[Query.Expression]
        def __init__(self, op: _Optional[_Union[Query.BooleanExpression.BooleanOperator, str]] = ..., arguments: _Optional[_Iterable[_Union[Query.Expression, _Mapping]]] = ...) -> None: ...
    class GroupByExpression(_message.Message):
        __slots__ = ["field", "derived"]
        FIELD_FIELD_NUMBER: _ClassVar[int]
        DERIVED_FIELD_NUMBER: _ClassVar[int]
        field: Query.FieldReference
        derived: Query.DerivedExpression
        def __init__(self, field: _Optional[_Union[Query.FieldReference, _Mapping]] = ..., derived: _Optional[_Union[Query.DerivedExpression, _Mapping]] = ...) -> None: ...
    class OrderByExpression(_message.Message):
        __slots__ = ["argument", "direction"]
        class Direction(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
            __slots__ = []
            ASC: _ClassVar[Query.OrderByExpression.Direction]
            DESC: _ClassVar[Query.OrderByExpression.Direction]
        ASC: Query.OrderByExpression.Direction
        DESC: Query.OrderByExpression.Direction
        ARGUMENT_FIELD_NUMBER: _ClassVar[int]
        DIRECTION_FIELD_NUMBER: _ClassVar[int]
        argument: Query.Expression
        direction: Query.OrderByExpression.Direction
        def __init__(self, argument: _Optional[_Union[Query.Expression, _Mapping]] = ..., direction: _Optional[_Union[Query.OrderByExpression.Direction, str]] = ...) -> None: ...
    class JoinExpression(_message.Message):
        __slots__ = ["joinType", "joinTable", "joinCondition", "tableAlias"]
        class JoinType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
            __slots__ = []
            LEFT_JOIN: _ClassVar[Query.JoinExpression.JoinType]
            INNER_JOIN: _ClassVar[Query.JoinExpression.JoinType]
            RIGHT_JOIN: _ClassVar[Query.JoinExpression.JoinType]
            FULL_OUTER_JOIN: _ClassVar[Query.JoinExpression.JoinType]
        LEFT_JOIN: Query.JoinExpression.JoinType
        INNER_JOIN: Query.JoinExpression.JoinType
        RIGHT_JOIN: Query.JoinExpression.JoinType
        FULL_OUTER_JOIN: Query.JoinExpression.JoinType
        JOINTYPE_FIELD_NUMBER: _ClassVar[int]
        JOINTABLE_FIELD_NUMBER: _ClassVar[int]
        JOINCONDITION_FIELD_NUMBER: _ClassVar[int]
        TABLEALIAS_FIELD_NUMBER: _ClassVar[int]
        joinType: Query.JoinExpression.JoinType
        joinTable: str
        joinCondition: Query.BooleanExpression
        tableAlias: str
        def __init__(self, joinType: _Optional[_Union[Query.JoinExpression.JoinType, str]] = ..., joinTable: _Optional[str] = ..., joinCondition: _Optional[_Union[Query.BooleanExpression, _Mapping]] = ..., tableAlias: _Optional[str] = ...) -> None: ...
    ID_FIELD_NUMBER: _ClassVar[int]
    SELECTFROM_FIELD_NUMBER: _ClassVar[int]
    SELECT_FIELD_NUMBER: _ClassVar[int]
    FILTER_FIELD_NUMBER: _ClassVar[int]
    GROUPBY_FIELD_NUMBER: _ClassVar[int]
    ORDERBY_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    DRYRUN_FIELD_NUMBER: _ClassVar[int]
    CLIENTVERSION_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    JOINS_FIELD_NUMBER: _ClassVar[int]
    TABLEALIAS_FIELD_NUMBER: _ClassVar[int]
    id: Query.Id
    selectFrom: str
    select: _containers.RepeatedCompositeFieldContainer[Query.SelectExpression]
    filter: Query.BooleanExpression
    groupBy: _containers.RepeatedCompositeFieldContainer[Query.GroupByExpression]
    orderBy: _containers.RepeatedCompositeFieldContainer[Query.OrderByExpression]
    limit: int
    dryRun: bool
    clientVersion: ClientVersion
    type: Query.Type
    joins: _containers.RepeatedCompositeFieldContainer[Query.JoinExpression]
    tableAlias: str
    def __init__(self, id: _Optional[_Union[Query.Id, _Mapping]] = ..., selectFrom: _Optional[str] = ..., select: _Optional[_Iterable[_Union[Query.SelectExpression, _Mapping]]] = ..., filter: _Optional[_Union[Query.BooleanExpression, _Mapping]] = ..., groupBy: _Optional[_Iterable[_Union[Query.GroupByExpression, _Mapping]]] = ..., orderBy: _Optional[_Iterable[_Union[Query.OrderByExpression, _Mapping]]] = ..., limit: _Optional[int] = ..., dryRun: bool = ..., clientVersion: _Optional[_Union[ClientVersion, _Mapping]] = ..., type: _Optional[_Union[Query.Type, str]] = ..., joins: _Optional[_Iterable[_Union[Query.JoinExpression, _Mapping]]] = ..., tableAlias: _Optional[str] = ...) -> None: ...

class QueryResult(_message.Message):
    __slots__ = ["queryString", "jsonData"]
    QUERYSTRING_FIELD_NUMBER: _ClassVar[int]
    JSONDATA_FIELD_NUMBER: _ClassVar[int]
    queryString: str
    jsonData: str
    def __init__(self, queryString: _Optional[str] = ..., jsonData: _Optional[str] = ...) -> None: ...

class ClientVersion(_message.Message):
    __slots__ = ["client", "datasetVersion", "codeVersion"]
    class Client(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
        DPM: _ClassVar[ClientVersion.Client]
        NODE_JS: _ClassVar[ClientVersion.Client]
        PYTHON: _ClassVar[ClientVersion.Client]
        CSHARP: _ClassVar[ClientVersion.Client]
    DPM: ClientVersion.Client
    NODE_JS: ClientVersion.Client
    PYTHON: ClientVersion.Client
    CSHARP: ClientVersion.Client
    CLIENT_FIELD_NUMBER: _ClassVar[int]
    DATASETVERSION_FIELD_NUMBER: _ClassVar[int]
    CODEVERSION_FIELD_NUMBER: _ClassVar[int]
    client: ClientVersion.Client
    datasetVersion: str
    codeVersion: str
    def __init__(self, client: _Optional[_Union[ClientVersion.Client, str]] = ..., datasetVersion: _Optional[str] = ..., codeVersion: _Optional[str] = ...) -> None: ...
