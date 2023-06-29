/**
 * Implements the Patch Dataset API backend interface.
 */
import { DateField, DateTimeField, DerivedField, LiteralField } from '../field';
import {
  AggregateFieldExpr,
  BooleanFieldExpr,
  DateTimeGranularity,
  FieldExpr,
  Operator,
  Scalar,
  UnaryBooleanFieldExpr,
} from '../field_expr';
import { Table } from '../table';
import { Backend } from './interface';

import { GraphQLClient, gql } from 'graphql-request';

type PatchOperator = Operator | 'before' | 'after';

// Helper functions.
/**
 * Returns input snake-cased string converted to camel-case.
 * @param snakeStr Snake-cased string to convert to camel-case.
 * @returns Camel-cased input string.
 */
function snakeToCamel(snakeStr: string): string {
  const parts = snakeStr.split('_').map((x) => x.toLowerCase());
  const first = parts[0];
  const rest = parts
    .slice(1)
    .map((w) => w.slice(0, 1).toUpperCase() + w.slice(1));
  return [first, ...rest].join('');
}

/**
 * Returns fieldName with an alias when present.
 * E.g.
 * ```typescript
 *   withAlias('salaryHighEarnersMax', 'highMax')
 * ```
 * returns `highMax: salaryHighEarnersMax`
 *
 * ```typescript
 *   withAlias('fooBar', undefined);
 * ```
 * returns `fooBar`
 *
 * @param fieldName
 * @param alias
 * @returns Aliased field name, when alias is defined; field name otherwise.
 */
function withAlias(fieldName: string, alias?: string): string {
  if (alias) {
    return `${alias}: ${fieldName}`;
  }

  return fieldName;
}

/**
 * Returns GraphQL representation of field suitable for use in Patch Dataset API call.
 * @param field Field expression.
 * @param useAlias Whether to use the field's alias if set.
 * @returns GraphQL representation of field suitable for use in Patch Dataset API call.
 */
function fieldAsGraphQL(field: FieldExpr, useAlias = false): string | null {
  if (field instanceof LiteralField) {
    let stringify = (x: Scalar): string => {
      if (typeof x === 'string' || x instanceof Date) {
        return `"${x}"`;
      } else {
        return x.toString();
      }
    };

    if (Array.isArray(field.value)) {
      return `[${field.value.map((x) => stringify(x)).join(', ')}]`;
    }
    return stringify(field.value);
  } else if (
    field instanceof AggregateFieldExpr ||
    field instanceof DerivedField
  ) {
    // E.g.
    // salary.max() returns 'salaryMax'
    // cancelledOn.year() returns 'cancelledOnYear'
    const baseField = field.operands()[0] as FieldExpr;
    const baseFieldGQL = fieldAsGraphQL(baseField, false); // Don't alias the base field.
    const fieldName = `${baseFieldGQL}${snakeToCamel('_' + field.operator())}`;
    return useAlias ? withAlias(fieldName, field.alias) : fieldName;
  } else if (field.operator() !== 'ident') {
    throw new Error(`Unexpected field expression ${field}`);
  }

  const fieldName = snakeToCamel(field.operands()[0].toString());
  return useAlias ? withAlias(fieldName, field.alias) : fieldName;
}

/**
 * Returns selected fields as GraphQL fragment suitable for use in Patch Dataset API call.
 * @param selection Selected fields.
 * @returns Selected fields as GraphQL fragment suitable for use in Patch Dataset API call.
 */
function selectionAsGraphQL(selection: FieldExpr[]): string {
  return selection
    .map((fieldExpr) => {
      try {
        return fieldAsGraphQL(fieldExpr, true); // Use alias in selection set.
      } catch (e) {
        // Unexpected selection fieldExpr.
        throw new Error(`Unexpected selection field expression ${fieldExpr}`);
      }
    })
    .join('\n');
}

/**
 * The default op formatter.
 * @param op
 * @param lhs
 * @param rhs
 * @returns The Dataset API graphQL boolean expression.
 */
function formatDefault(
  op: PatchOperator,
  lhs: FieldExpr,
  rhs: LiteralField<any>
): string {
  return `{
    ${fieldAsGraphQL(lhs as FieldExpr)}: {
      ${op}: ${fieldAsGraphQL(rhs as FieldExpr)}
    }
  }`;
}

/**
 * Formats the `inPast` operator and its operands to the equivalent Patch Dataset API operator.
 * @param _op Unused operator
 * @param lhs The LHS of the expression.
 * @param rhs The RHS LiteralField of the expression.
 * @returns The formatted graphQL boolean expression equivalent of the `inPast` operator.
 */
function formatInPast(
  _op: Operator,
  lhs: FieldExpr,
  rhs: LiteralField<any>
): string {
  // rhs must be of form [<number>, <number>, TimeGranularity]
  if (!Array.isArray(rhs.value) || rhs.value.length != 3) {
    throw new Error(
      `Patch error: inPast specified with invalid arguments ${rhs.value}, must have [<number>, <number>, <granularity>]`
    );
  }
  let olderThan: number = rhs.value[0];
  let newerThan: number = rhs.value[1];
  const granularity: DateTimeGranularity = rhs.value[2];
  return `{
    ${fieldAsGraphQL(lhs as FieldExpr)}: {
      olderThan: {${granularity}: ${olderThan}},
      newerThan: {${granularity}: ${newerThan}},
    }
  }`;
}

/**
 * Formats a boolean expression with a temporal (DateField, DateTimeField) LHS.
 * We need to handle such expressions specially because Patch's Dataset API
 * specifies different operators (before, after) for temporal field comparisons
 * than the usual (<, >) operators.
 * @param op operator
 * @param lhs The LHS of the expression.
 * @param rhs The RHS LiteralField of the expression.
 * @returns The formatted graphQL boolean expression for temporal fields.
 */
function formatTemporal(
  op: Operator,
  lhs: FieldExpr,
  rhs: LiteralField<any>
): string {
  let temporalOp: PatchOperator = op;
  if (op === 'lt' || op === 'lte') {
    temporalOp = 'before';
  } else if (op === 'gt' || op === 'gte') {
    temporalOp = 'after';
  }

  // TODO(3290): Handle eq, neq. Patch Dataset API does not yet support these temporal fields.

  return formatDefault(temporalOp, lhs, rhs);
}

function formatUnary(
  op: Operator,
  operand: FieldExpr,
  _rhs: LiteralField<any>
): string {
  let op_ = op === 'isNull' ? 'eq' : 'neq';

  return `{
      ${fieldAsGraphQL(operand as FieldExpr)}: {
        ${op_}: null
      }
    }`;
}

/**
 * Returns a formatter function that formats a boolean expression with operator op.
 * @param op Operator
 * @param lhs The field expression that's the left-hand side of the boolean expression.
 * @returns Function to format a boolean expression for the specific operator.
 */
function getOpFormatter(
  op: Operator,
  lhs: FieldExpr
): (op: Operator, lhs: FieldExpr, rhs: LiteralField<any>) => string {
  if (op === 'inPast') {
    return formatInPast;
  }

  // Handle some DateField and DateTimeField operations specially.
  if (lhs instanceof DateField || lhs instanceof DateTimeField) {
    return formatTemporal;
  }

  // Handle if op is unaryOperator.
  if (op === 'isNull' || op === 'isNotNull') {
    return formatUnary;
  }
  // Return the default formatter.
  return formatDefault;
}

/**
 * Converts Boolean expression to GraphQL value suitable for use in the filter clause of a
 * Patch Dataset API call.
 * @param expr Boolean expression to convert to GraphQL value suitable for use in
 *    the filter clause of a Patch Dataset API call.
 * @returns GraphQL value for use in the filter clause of a Patch Dataset API call.
 */
function exprAsGraphQL(expr: BooleanFieldExpr | UnaryBooleanFieldExpr): string {
  let op = expr.operator();
  if (op === 'and' || op === 'or') {
    return `{
      ${op}: [${expr
      .operands()
      .map((e) => exprAsGraphQL(e as BooleanFieldExpr))
      .join(',\n')}]
    }`;
  }
  let [lhs, rhs] = expr.operands();
  if (expr instanceof UnaryBooleanFieldExpr) {
    // UnaryBooleanFieldExpr has no RHS, so create one to appease the type checks below.
    // The unary operator formatter ignores the rhs anyway.
    rhs = new LiteralField(0);
  }
  // Patch supports only literals in the RHS.
  if (!(rhs instanceof LiteralField)) {
    throw new Error(
      `Patch error: non-literal RHS not supported in expression: ${lhs.toString()} ${op} ${rhs.toString()}`
    );
  }

  const formatter = getOpFormatter(op, lhs as FieldExpr);
  return formatter(op, lhs as FieldExpr, rhs);

  // TODO(PAT-3175): handle UnaryFieldExpr here. Difficulty: not directly supported by Dataset API.
}

function queryNameAsGraphQL(name: string): string {
  return `${snakeToCamel(name)}Query`;
}

export class Patch implements Backend {
  constructor(
    private path: string,
    private datasetName: string,
    private version: string,
    private authToken: string
  ) {}

  /**
   * Compiles query object into GraphQL query string for use in a Patch Dataset API call.
   * @param query query to compile
   * @returns Compiled GraphQL query string for use in a Patch Dataset API call.
   */
  async compile(query: Table): Promise<string> {
    const queryName = queryNameAsGraphQL(query.name);
    const {
      filterExpr: filter,
      selection,
      ordering: orderBy,
      limitTo: limit,
    } = query;
    if (!selection) {
      throw new Error('Queries to patch must include a selection');
    }

    let selectionFragment = selectionAsGraphQL(selection);

    let paramParts: string[] = [];
    if (filter) {
      paramParts.push(`filter: ${exprAsGraphQL(filter)}`);
    }

    if (orderBy) {
      let ordering = orderBy
        .map(([field, dir]) => {
          return `{${fieldAsGraphQL(field)}: ${dir.toLowerCase()}}`;
        })
        .join(', ');
      paramParts.push(`orderBy: [${ordering}]`);
    }

    paramParts.push(`limit: ${limit}`);

    const compiledQuery = `${queryName}(${paramParts.join(', ')}) {
      ${selectionFragment}
    }`;
    return Promise.resolve(compiledQuery);
  }

  /**
   * Executes query using Patch's Dataset API and returns promise with result rows.
   * @param query Query to execute.
   * @returns Result rows.
   */
  async execute<Row>(query: Table): Promise<Row[]> {
    let sourcePath = query.source;
    if (!sourcePath) {
      throw new Error(
        'Cannot execute query whose table does not have a source specified'
      );
    }

    let compiledQuery = await this.compile(query);

    // Issue gql call to patch backend.
    const graphQLClient = new GraphQLClient(sourcePath, {
      headers: {
        authorization: `Bearer ${this.authToken}`,
      },
    });

    const data: { [key: string]: any } = await graphQLClient.request(
      gql`{${compiledQuery}}`
    );
    // Extract the query data.
    const queryName = queryNameAsGraphQL(query.name);
    return data[queryName];
  }
}
