import { Ordering, Table } from '../../table';
import { Backend } from '../interface';

import { ChannelCredentials, ServiceError } from '@grpc/grpc-js';
import { DerivedField, LiteralField } from '../../field';
import {
  AggregateFieldExpr,
  AggregateOperator,
  BooleanFieldExpr,
  BooleanOperator,
  FieldExpr,
  ProjectionOperator,
  Scalar,
} from '../../fieldExpr';
import { DpmAgentClient as DpmAgentGrpcClient } from './dpm_agent_grpc_pb';
import {
  CompiledQuery,
  ConnectionRequest,
  ConnectionResponse,
  Query as DpmAgentQuery,
  QueryResult,
} from './dpm_agent_pb';

function makeDpmLiteral(literal: LiteralField<Scalar>): DpmAgentQuery.Literal {
  let makeLiteral = (x: Scalar): DpmAgentQuery.Literal => {
    const dpmLit = new DpmAgentQuery.Literal();
    if (typeof x === 'string') {
      return dpmLit.setString(x);
    } else if (typeof x === 'number') {
      return Number.isInteger(x) ? dpmLit.setI64(x) : dpmLit.setF64(x);
    } else if (typeof x === 'boolean') {
      return dpmLit.setBoolean(x);
    }

    // Must be a Date type.
    return dpmLit.setTimestammp(+x);
  };

  if (Array.isArray(literal.value)) {
    return new DpmAgentQuery.Literal().setList(
      new DpmAgentQuery.Literal.List().setValuesList(literal.value.map(makeLiteral))
    );
  }
  return makeLiteral(literal.value);
}

function makeDpmFieldReference(field: FieldExpr): DpmAgentQuery.FieldReference {
  return new DpmAgentQuery.FieldReference().setFieldname(
    field.operands()[0].toString()
  );
}

const aggregateOperatorMap = {
  min: DpmAgentQuery.AggregateExpression.AggregateOperator.MIN,
  max: DpmAgentQuery.AggregateExpression.AggregateOperator.MAX,
  count: DpmAgentQuery.AggregateExpression.AggregateOperator.COUNT,
  countDistinct: DpmAgentQuery.AggregateExpression.AggregateOperator.COUNT_DISTINCT,
  avg: DpmAgentQuery.AggregateExpression.AggregateOperator.MEAN,
  avgDistinct: DpmAgentQuery.AggregateExpression.AggregateOperator.MEAN, // dpm-agent uses Ibis, which does not support distinct mean.
};

function makeDpmAggregateExpression(
  aggExpr: AggregateFieldExpr<Scalar>
): DpmAgentQuery.AggregateExpression {
  const baseField = aggExpr.operands()[0] as FieldExpr;
  const baseDpmExpr = makeDpmExpression(baseField);
  const aggOp = aggExpr.operator() as AggregateOperator;
  const dpmAggOp = aggregateOperatorMap[aggOp];
  if (dpmAggOp === undefined) {
    throw new Error(`Unsupported aggregate operation ${aggOp}`);
  }

  return new DpmAgentQuery.AggregateExpression()
    .setArgument(baseDpmExpr)
    .setOp(dpmAggOp);
}

const projectionOperatorMap = {
  day: DpmAgentQuery.DerivedExpression.ProjectionOperator.DAY,
  month: DpmAgentQuery.DerivedExpression.ProjectionOperator.MONTH,
  year: DpmAgentQuery.DerivedExpression.ProjectionOperator.YEAR,
  hour: DpmAgentQuery.DerivedExpression.ProjectionOperator.HOUR,
  minute: DpmAgentQuery.DerivedExpression.ProjectionOperator.MINUTE,
  second: DpmAgentQuery.DerivedExpression.ProjectionOperator.SECOND,
  millisecond: DpmAgentQuery.DerivedExpression.ProjectionOperator.MILLISECOND,
};

function makeDpmDerivedExpression(
  derivedField: DerivedField<Scalar, Scalar>
): DpmAgentQuery.DerivedExpression {
  const baseField = derivedField.operands()[0] as FieldExpr;
  const baseDpmExpr = makeDpmExpression(baseField);
  const projectionOp = derivedField.operator() as ProjectionOperator;
  const dpmProjectionOp = projectionOperatorMap[projectionOp];
  if (projectionOp === undefined) {
    throw new Error(`Unsupported projection operation ${projectionOp}`);
  }

  return new DpmAgentQuery.DerivedExpression()
    .setArgument(baseDpmExpr)
    .setOp(dpmProjectionOp);
}

function makeDpmExpression(field: FieldExpr): DpmAgentQuery.Expression {
  if (field instanceof LiteralField) {
    return new DpmAgentQuery.Expression().setLiteral(makeDpmLiteral(field));
  } else if (field instanceof AggregateFieldExpr) {
    return new DpmAgentQuery.Expression().setAggregate(
      makeDpmAggregateExpression(field)
    );
  } else if (field instanceof DerivedField) {
    return new DpmAgentQuery.Expression().setDerived(
      makeDpmDerivedExpression(field)
    );
  } else if (field.operator() !== 'ident') {
    throw new Error(`Unexpected field expression ${field}`);
  }
  return new DpmAgentQuery.Expression().setField(makeDpmFieldReference(field));
}

function makeDpmGroupByExpression(
  field: FieldExpr
): DpmAgentQuery.GroupByExpression {
  if (field instanceof DerivedField) {
    return new DpmAgentQuery.GroupByExpression().setDerived(
      makeDpmDerivedExpression(field)
    );
  } else if (field.operator() !== 'ident') {
    throw new Error(`Unexpected field expression in groupBy: ${field}`);
  }
  return new DpmAgentQuery.GroupByExpression().setField(
    makeDpmFieldReference(field)
  );
}

function makeDpmSelectExpression(
  field: FieldExpr
): DpmAgentQuery.SelectExpression {
  const selectExpr = new DpmAgentQuery.SelectExpression().setArgument(
    makeDpmExpression(field)
  );
  if (field.alias !== undefined) {
    return selectExpr.setAlias(field.alias);
  }
  return selectExpr;
}

const booleanOperatorMap = {
  and: DpmAgentQuery.BooleanExpression.BooleanOperator.AND,
  or: DpmAgentQuery.BooleanExpression.BooleanOperator.OR,
  eq: DpmAgentQuery.BooleanExpression.BooleanOperator.EQ,
  neq: DpmAgentQuery.BooleanExpression.BooleanOperator.NEQ,
  gt: DpmAgentQuery.BooleanExpression.BooleanOperator.GT,
  gte: DpmAgentQuery.BooleanExpression.BooleanOperator.GTE,
  lt: DpmAgentQuery.BooleanExpression.BooleanOperator.LT,
  lte: DpmAgentQuery.BooleanExpression.BooleanOperator.LTE,
  like: DpmAgentQuery.BooleanExpression.BooleanOperator.LIKE,
  between: DpmAgentQuery.BooleanExpression.BooleanOperator.BETWEEN,
  in: DpmAgentQuery.BooleanExpression.BooleanOperator.IN,
  // TODO(PAT-3175, PAT-3176): Define once we support unary not.
  not: undefined,
  // TODO(PAT-3355): Remove `inPast` once we redefine it in terms of a `between` check.
  inPast: undefined,
};

function makeDpmBooleanExpression(
  filter: BooleanFieldExpr
): DpmAgentQuery.BooleanExpression {
  const BooleanOperator = DpmAgentQuery.BooleanExpression.BooleanOperator;
  let op = filter.operator();
  if (op === 'and' || op === 'or') {
    const args = filter.operands().map((expr) => {
      const boolExpr = makeDpmBooleanExpression(expr as BooleanFieldExpr);
      return new DpmAgentQuery.Expression().setCondition(boolExpr);
    });
    return new DpmAgentQuery.BooleanExpression()
      .setOp(booleanOperatorMap[op])
      .setArgumentsList(args);
  }

  const dpmBooleanOp = booleanOperatorMap[op as BooleanOperator];
  if (dpmBooleanOp === undefined) {
    throw new Error(`Unhandled boolean operator ${op}`);
  }

  const args = filter
    .operands()
    .map((expr) => makeDpmExpression(expr as FieldExpr));
  return new DpmAgentQuery.BooleanExpression()
    .setOp(dpmBooleanOp)
    .setArgumentsList(args);
}


function makeDpmOrderByExpression(
  ordering: Ordering
): DpmAgentQuery.OrderByExpression {
  const [fieldExpr, direction] = ordering;
  return new DpmAgentQuery.OrderByExpression()
    .setArgument(makeDpmExpression(fieldExpr))
    .setDirection(
      direction === 'ASC'
        ? DpmAgentQuery.OrderByExpression.Direction.ASC
        : DpmAgentQuery.OrderByExpression.Direction.DESC
    );
}

export class DpmAgentClient implements Backend {
  private client: DpmAgentGrpcClient;
  private connectionId: Promise<string>;

  private async makeDpmAgentQuery(query: Table): Promise<DpmAgentQuery> {
    const dpmAgentQuery = new DpmAgentQuery();
    dpmAgentQuery.setConnectionid(await this.connectionId);
    dpmAgentQuery.setSelectfrom(query.name);

    const {
      filterExpr: filter,
      selection,
      ordering: orderBy,
      limitTo: limit,
    } = query;
    const selections = selection?.map(makeDpmSelectExpression);
    if (selections) {
      dpmAgentQuery.setSelectList(selections);
    }

    // Process filter.
    if (filter) {
      dpmAgentQuery.setFilter(makeDpmBooleanExpression(filter));
    }

    // Process any groupings defined in selection.
    if (selection?.findIndex((fieldExpr) => fieldExpr instanceof AggregateFieldExpr) !== -1) {
      const grouping = selection?.filter((fieldExpr) => !(fieldExpr instanceof AggregateFieldExpr));
      if (grouping) {
        dpmAgentQuery.setGroupbyList(grouping.map(makeDpmGroupByExpression));
      }
    }

    // Process orderBy.
    if (orderBy !== undefined && orderBy.length > 0) {
      const dpmOrderings = orderBy.map(makeDpmOrderByExpression);
      dpmAgentQuery.setOrderbyList(dpmOrderings);
    }

    if (limit > 0) {
      dpmAgentQuery.setLimit(limit);
    }

    return Promise.resolve(dpmAgentQuery);
  }

  constructor(
    serviceAddress: string,
    creds: ChannelCredentials,
    connectionRequest: ConnectionRequest
  ) {
    console.log('Attempting to connect to', serviceAddress);
    this.client = new DpmAgentGrpcClient(serviceAddress, creds);
    this.connectionId = new Promise((resolve, reject) => {
      this.client.connect(
        connectionRequest,
        (error: ServiceError | null, response: ConnectionResponse) => {
          if (error) {
            console.log('dpm-agent client: Error connecting...', error);
            reject(new Error('Error connecting', { cause: error }));
          } else {
            console.log(
              `dpm-agent client: Connected, connection id: ${response.getConnectionid()}`
            );
            resolve(response.getConnectionid());
          }
        }
      );
    });
  }

  async compile(query: Table): Promise<string> {
    const dpmAgentQuery = await this.makeDpmAgentQuery(query);
    return new Promise((resolve, reject) => {
      this.client.compile(
        dpmAgentQuery,
        (error: ServiceError | null, response: CompiledQuery) => {
          if (error) {
            console.log('dpm-agent client: Error compiling query...', error);
            reject(new Error('Error compiling query', { cause: error }));
          } else {
            resolve(response.getResult());
          }
        }
      );
    });
  }

  async execute<Row>(query: Table): Promise<Row[]> {
    const dpmAgentQuery = await this.makeDpmAgentQuery(query);
    return new Promise((resolve, reject) => {
      this.client.execute(
        dpmAgentQuery,
        (error: ServiceError | null, response: QueryResult) => {
          if (error) {
            console.log('dpm-agent client: Error executing query...', error);
            reject(new Error('Error executing query', { cause: error }));
          } else {
            let jsonData: Row[] = [];
            try {
              jsonData = JSON.parse(response.getJsondata());
            } catch (e) {
              console.log('dpm-agent: Error parsing results', e);
              reject(new Error('Error parsing JSON', { cause: e }));
            }
            resolve(jsonData);
          }
        }
      );
    });
  }
}
