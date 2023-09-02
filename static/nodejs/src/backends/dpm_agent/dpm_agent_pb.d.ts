// package: dpm_agent
// file: dpm_agent.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";

export class Query extends jspb.Message { 

    hasId(): boolean;
    clearId(): void;
    getId(): Query.Id | undefined;
    setId(value?: Query.Id): Query;
    getSelectfrom(): string;
    setSelectfrom(value: string): Query;
    clearSelectList(): void;
    getSelectList(): Array<Query.SelectExpression>;
    setSelectList(value: Array<Query.SelectExpression>): Query;
    addSelect(value?: Query.SelectExpression, index?: number): Query.SelectExpression;

    hasFilter(): boolean;
    clearFilter(): void;
    getFilter(): Query.BooleanExpression | undefined;
    setFilter(value?: Query.BooleanExpression): Query;
    clearGroupbyList(): void;
    getGroupbyList(): Array<Query.GroupByExpression>;
    setGroupbyList(value: Array<Query.GroupByExpression>): Query;
    addGroupby(value?: Query.GroupByExpression, index?: number): Query.GroupByExpression;
    clearOrderbyList(): void;
    getOrderbyList(): Array<Query.OrderByExpression>;
    setOrderbyList(value: Array<Query.OrderByExpression>): Query;
    addOrderby(value?: Query.OrderByExpression, index?: number): Query.OrderByExpression;

    hasLimit(): boolean;
    clearLimit(): void;
    getLimit(): number | undefined;
    setLimit(value: number): Query;

    hasDryrun(): boolean;
    clearDryrun(): void;
    getDryrun(): boolean | undefined;
    setDryrun(value: boolean): Query;

    hasClientversion(): boolean;
    clearClientversion(): void;
    getClientversion(): ClientVersion | undefined;
    setClientversion(value?: ClientVersion): Query;

    hasType(): boolean;
    clearType(): void;
    getType(): Query.Type | undefined;
    setType(value: Query.Type): Query;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Query.AsObject;
    static toObject(includeInstance: boolean, msg: Query): Query.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Query, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Query;
    static deserializeBinaryFromReader(message: Query, reader: jspb.BinaryReader): Query;
}

export namespace Query {
    export type AsObject = {
        id?: Query.Id.AsObject,
        selectfrom: string,
        selectList: Array<Query.SelectExpression.AsObject>,
        filter?: Query.BooleanExpression.AsObject,
        groupbyList: Array<Query.GroupByExpression.AsObject>,
        orderbyList: Array<Query.OrderByExpression.AsObject>,
        limit?: number,
        dryrun?: boolean,
        clientversion?: ClientVersion.AsObject,
        type?: Query.Type,
    }


    export class Id extends jspb.Message { 

        hasPackageid(): boolean;
        clearPackageid(): void;
        getPackageid(): string;
        setPackageid(value: string): Id;

        hasSourceid(): boolean;
        clearSourceid(): void;
        getSourceid(): string;
        setSourceid(value: string): Id;

        getIdTypeCase(): Id.IdTypeCase;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Id.AsObject;
        static toObject(includeInstance: boolean, msg: Id): Id.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: Id, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Id;
        static deserializeBinaryFromReader(message: Id, reader: jspb.BinaryReader): Id;
    }

    export namespace Id {
        export type AsObject = {
            packageid: string,
            sourceid: string,
        }

        export enum IdTypeCase {
            ID_TYPE_NOT_SET = 0,
            PACKAGEID = 1,
            SOURCEID = 2,
        }

    }

    export class SelectExpression extends jspb.Message { 

        hasArgument(): boolean;
        clearArgument(): void;
        getArgument(): Query.Expression | undefined;
        setArgument(value?: Query.Expression): SelectExpression;

        hasAlias(): boolean;
        clearAlias(): void;
        getAlias(): string | undefined;
        setAlias(value: string): SelectExpression;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): SelectExpression.AsObject;
        static toObject(includeInstance: boolean, msg: SelectExpression): SelectExpression.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: SelectExpression, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): SelectExpression;
        static deserializeBinaryFromReader(message: SelectExpression, reader: jspb.BinaryReader): SelectExpression;
    }

    export namespace SelectExpression {
        export type AsObject = {
            argument?: Query.Expression.AsObject,
            alias?: string,
        }
    }

    export class Expression extends jspb.Message { 

        hasField(): boolean;
        clearField(): void;
        getField(): Query.FieldReference | undefined;
        setField(value?: Query.FieldReference): Expression;

        hasLiteral(): boolean;
        clearLiteral(): void;
        getLiteral(): Query.Literal | undefined;
        setLiteral(value?: Query.Literal): Expression;

        hasDerived(): boolean;
        clearDerived(): void;
        getDerived(): Query.DerivedExpression | undefined;
        setDerived(value?: Query.DerivedExpression): Expression;

        hasAggregate(): boolean;
        clearAggregate(): void;
        getAggregate(): Query.AggregateExpression | undefined;
        setAggregate(value?: Query.AggregateExpression): Expression;

        hasCondition(): boolean;
        clearCondition(): void;
        getCondition(): Query.BooleanExpression | undefined;
        setCondition(value?: Query.BooleanExpression): Expression;

        getExTypeCase(): Expression.ExTypeCase;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Expression.AsObject;
        static toObject(includeInstance: boolean, msg: Expression): Expression.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: Expression, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Expression;
        static deserializeBinaryFromReader(message: Expression, reader: jspb.BinaryReader): Expression;
    }

    export namespace Expression {
        export type AsObject = {
            field?: Query.FieldReference.AsObject,
            literal?: Query.Literal.AsObject,
            derived?: Query.DerivedExpression.AsObject,
            aggregate?: Query.AggregateExpression.AsObject,
            condition?: Query.BooleanExpression.AsObject,
        }

        export enum ExTypeCase {
            EX_TYPE_NOT_SET = 0,
            FIELD = 1,
            LITERAL = 2,
            DERIVED = 3,
            AGGREGATE = 4,
            CONDITION = 5,
        }

    }

    export class FieldReference extends jspb.Message { 
        getFieldname(): string;
        setFieldname(value: string): FieldReference;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): FieldReference.AsObject;
        static toObject(includeInstance: boolean, msg: FieldReference): FieldReference.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: FieldReference, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): FieldReference;
        static deserializeBinaryFromReader(message: FieldReference, reader: jspb.BinaryReader): FieldReference;
    }

    export namespace FieldReference {
        export type AsObject = {
            fieldname: string,
        }
    }

    export class Literal extends jspb.Message { 

        hasString(): boolean;
        clearString(): void;
        getString(): string;
        setString(value: string): Literal;

        hasBoolean(): boolean;
        clearBoolean(): void;
        getBoolean(): boolean;
        setBoolean(value: boolean): Literal;

        hasUi32(): boolean;
        clearUi32(): void;
        getUi32(): number;
        setUi32(value: number): Literal;

        hasUi64(): boolean;
        clearUi64(): void;
        getUi64(): number;
        setUi64(value: number): Literal;

        hasI32(): boolean;
        clearI32(): void;
        getI32(): number;
        setI32(value: number): Literal;

        hasI64(): boolean;
        clearI64(): void;
        getI64(): number;
        setI64(value: number): Literal;

        hasF32(): boolean;
        clearF32(): void;
        getF32(): number;
        setF32(value: number): Literal;

        hasF64(): boolean;
        clearF64(): void;
        getF64(): number;
        setF64(value: number): Literal;

        hasTimestamp(): boolean;
        clearTimestamp(): void;
        getTimestamp(): number;
        setTimestamp(value: number): Literal;

        hasList(): boolean;
        clearList(): void;
        getList(): Query.Literal.List | undefined;
        setList(value?: Query.Literal.List): Literal;

        getLiteralTypeCase(): Literal.LiteralTypeCase;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): Literal.AsObject;
        static toObject(includeInstance: boolean, msg: Literal): Literal.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: Literal, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): Literal;
        static deserializeBinaryFromReader(message: Literal, reader: jspb.BinaryReader): Literal;
    }

    export namespace Literal {
        export type AsObject = {
            string: string,
            pb_boolean: boolean,
            ui32: number,
            ui64: number,
            i32: number,
            i64: number,
            f32: number,
            f64: number,
            timestamp: number,
            list?: Query.Literal.List.AsObject,
        }


        export class List extends jspb.Message { 
            clearValuesList(): void;
            getValuesList(): Array<Query.Literal>;
            setValuesList(value: Array<Query.Literal>): List;
            addValues(value?: Query.Literal, index?: number): Query.Literal;

            serializeBinary(): Uint8Array;
            toObject(includeInstance?: boolean): List.AsObject;
            static toObject(includeInstance: boolean, msg: List): List.AsObject;
            static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
            static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
            static serializeBinaryToWriter(message: List, writer: jspb.BinaryWriter): void;
            static deserializeBinary(bytes: Uint8Array): List;
            static deserializeBinaryFromReader(message: List, reader: jspb.BinaryReader): List;
        }

        export namespace List {
            export type AsObject = {
                valuesList: Array<Query.Literal.AsObject>,
            }
        }


        export enum LiteralTypeCase {
            LITERAL_TYPE_NOT_SET = 0,
            STRING = 1,
            BOOLEAN = 2,
            UI32 = 3,
            UI64 = 4,
            I32 = 5,
            I64 = 6,
            F32 = 7,
            F64 = 8,
            TIMESTAMP = 9,
            LIST = 10,
        }

    }

    export class DerivedExpression extends jspb.Message { 
        getOp(): Query.DerivedExpression.ProjectionOperator;
        setOp(value: Query.DerivedExpression.ProjectionOperator): DerivedExpression;

        hasArgument(): boolean;
        clearArgument(): void;
        getArgument(): Query.Expression | undefined;
        setArgument(value?: Query.Expression): DerivedExpression;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): DerivedExpression.AsObject;
        static toObject(includeInstance: boolean, msg: DerivedExpression): DerivedExpression.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: DerivedExpression, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): DerivedExpression;
        static deserializeBinaryFromReader(message: DerivedExpression, reader: jspb.BinaryReader): DerivedExpression;
    }

    export namespace DerivedExpression {
        export type AsObject = {
            op: Query.DerivedExpression.ProjectionOperator,
            argument?: Query.Expression.AsObject,
        }

        export enum ProjectionOperator {
    YEAR = 0,
    MONTH = 1,
    DAY = 2,
    HOUR = 3,
    MINUTE = 4,
    SECOND = 5,
    MILLISECOND = 6,
    DATE = 7,
    TIME = 8,
        }

    }

    export class AggregateExpression extends jspb.Message { 
        getOp(): Query.AggregateExpression.AggregateOperator;
        setOp(value: Query.AggregateExpression.AggregateOperator): AggregateExpression;

        hasArgument(): boolean;
        clearArgument(): void;
        getArgument(): Query.Expression | undefined;
        setArgument(value?: Query.Expression): AggregateExpression;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): AggregateExpression.AsObject;
        static toObject(includeInstance: boolean, msg: AggregateExpression): AggregateExpression.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: AggregateExpression, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): AggregateExpression;
        static deserializeBinaryFromReader(message: AggregateExpression, reader: jspb.BinaryReader): AggregateExpression;
    }

    export namespace AggregateExpression {
        export type AsObject = {
            op: Query.AggregateExpression.AggregateOperator,
            argument?: Query.Expression.AsObject,
        }

        export enum AggregateOperator {
    MIN = 0,
    MAX = 1,
    MEAN = 2,
    MEDIAN = 3,
    COUNT = 4,
    COUNT_DISTINCT = 5,
    SUM = 6,
        }

    }

    export class BooleanExpression extends jspb.Message { 
        getOp(): Query.BooleanExpression.BooleanOperator;
        setOp(value: Query.BooleanExpression.BooleanOperator): BooleanExpression;
        clearArgumentsList(): void;
        getArgumentsList(): Array<Query.Expression>;
        setArgumentsList(value: Array<Query.Expression>): BooleanExpression;
        addArguments(value?: Query.Expression, index?: number): Query.Expression;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): BooleanExpression.AsObject;
        static toObject(includeInstance: boolean, msg: BooleanExpression): BooleanExpression.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: BooleanExpression, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): BooleanExpression;
        static deserializeBinaryFromReader(message: BooleanExpression, reader: jspb.BinaryReader): BooleanExpression;
    }

    export namespace BooleanExpression {
        export type AsObject = {
            op: Query.BooleanExpression.BooleanOperator,
            argumentsList: Array<Query.Expression.AsObject>,
        }

        export enum BooleanOperator {
    AND = 0,
    OR = 1,
    EQ = 2,
    NEQ = 3,
    LT = 4,
    LTE = 5,
    GT = 6,
    GTE = 7,
    LIKE = 8,
    BETWEEN = 9,
    IN = 10,
    IS_NULL = 11,
    IS_NOT_NULL = 12,
        }

    }

    export class GroupByExpression extends jspb.Message { 

        hasField(): boolean;
        clearField(): void;
        getField(): Query.FieldReference | undefined;
        setField(value?: Query.FieldReference): GroupByExpression;

        hasDerived(): boolean;
        clearDerived(): void;
        getDerived(): Query.DerivedExpression | undefined;
        setDerived(value?: Query.DerivedExpression): GroupByExpression;

        getExTypeCase(): GroupByExpression.ExTypeCase;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): GroupByExpression.AsObject;
        static toObject(includeInstance: boolean, msg: GroupByExpression): GroupByExpression.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: GroupByExpression, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): GroupByExpression;
        static deserializeBinaryFromReader(message: GroupByExpression, reader: jspb.BinaryReader): GroupByExpression;
    }

    export namespace GroupByExpression {
        export type AsObject = {
            field?: Query.FieldReference.AsObject,
            derived?: Query.DerivedExpression.AsObject,
        }

        export enum ExTypeCase {
            EX_TYPE_NOT_SET = 0,
            FIELD = 1,
            DERIVED = 2,
        }

    }

    export class OrderByExpression extends jspb.Message { 

        hasArgument(): boolean;
        clearArgument(): void;
        getArgument(): Query.Expression | undefined;
        setArgument(value?: Query.Expression): OrderByExpression;

        hasDirection(): boolean;
        clearDirection(): void;
        getDirection(): Query.OrderByExpression.Direction | undefined;
        setDirection(value: Query.OrderByExpression.Direction): OrderByExpression;

        serializeBinary(): Uint8Array;
        toObject(includeInstance?: boolean): OrderByExpression.AsObject;
        static toObject(includeInstance: boolean, msg: OrderByExpression): OrderByExpression.AsObject;
        static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
        static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
        static serializeBinaryToWriter(message: OrderByExpression, writer: jspb.BinaryWriter): void;
        static deserializeBinary(bytes: Uint8Array): OrderByExpression;
        static deserializeBinaryFromReader(message: OrderByExpression, reader: jspb.BinaryReader): OrderByExpression;
    }

    export namespace OrderByExpression {
        export type AsObject = {
            argument?: Query.Expression.AsObject,
            direction?: Query.OrderByExpression.Direction,
        }

        export enum Direction {
    ASC = 0,
    DESC = 1,
        }

    }


    export enum Type {
    DATA = 0,
    INTROSPECTION = 1,
    }

}

export class QueryResult extends jspb.Message { 
    getQuerystring(): string;
    setQuerystring(value: string): QueryResult;
    getJsondata(): string;
    setJsondata(value: string): QueryResult;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): QueryResult.AsObject;
    static toObject(includeInstance: boolean, msg: QueryResult): QueryResult.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: QueryResult, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): QueryResult;
    static deserializeBinaryFromReader(message: QueryResult, reader: jspb.BinaryReader): QueryResult;
}

export namespace QueryResult {
    export type AsObject = {
        querystring: string,
        jsondata: string,
    }
}

export class ClientVersion extends jspb.Message { 
    getClient(): ClientVersion.Client;
    setClient(value: ClientVersion.Client): ClientVersion;
    getDatasetversion(): string;
    setDatasetversion(value: string): ClientVersion;
    getCodeversion(): string;
    setCodeversion(value: string): ClientVersion;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ClientVersion.AsObject;
    static toObject(includeInstance: boolean, msg: ClientVersion): ClientVersion.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: ClientVersion, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ClientVersion;
    static deserializeBinaryFromReader(message: ClientVersion, reader: jspb.BinaryReader): ClientVersion;
}

export namespace ClientVersion {
    export type AsObject = {
        client: ClientVersion.Client,
        datasetversion: string,
        codeversion: string,
    }

    export enum Client {
    DPM = 0,
    NODE_JS = 1,
    PYTHON = 2,
    CSHARP = 3,
    }

}
