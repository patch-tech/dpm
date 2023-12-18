import { describe, expect, test } from '@jest/globals';
import { makeDpmAgentQuery } from '../src/backends/dpm_agent/dpm_agent_client';
import {
  ClientVersion,
  Query as DpmAgentQuery,
} from '../src/backends/dpm_agent/dpm_agent_pb';
import { Backend } from '../src/backends/interface';
import { DateField, Field, StringField } from '../src/field';
import { Table } from '../src/table';

class TestBackend implements Backend {
  compile(_query: Table): Promise<string> {
    return Promise.resolve('SELECT * from "foo"');
  }

  execute<Row extends object>(_query: Table): Promise<Row[]> {
    let res: Row[] = [];
    return Promise.resolve(res);
  }
}

describe('makeDpmAgentQuery', () => {
  const backend = new TestBackend();

  const id = new StringField('id');
  const name = new StringField('name');
  const price = new Field<number>('price');
  const createdOn = new DateField('created_on');
  const table = new Table({
    backend,
    packageId: 'pkg-123',
    datasetName: 'ds-456',
    datasetVersion: '0.1.0',
    source: 'test',
    name: 'testTable',
    fields: [id, name, price, createdOn],
  });

  test('returns expected Query message for a query with selections only', () => {
    const query = table.select('id', 'name').limit(10);
    const dpmQuery = makeDpmAgentQuery(query);
    const want: DpmAgentQuery.AsObject = {
      id: {
        packageid: 'pkg-123',
        sourceid: '',
      },
      clientversion: {
        client: ClientVersion.Client.NODE_JS,
        datasetversion: '0.1.0',
        codeversion: '0.1.0',
      },
      selectfrom: 'testTable',
      selectList: [
        {
          argument: {
            field: {
              fieldname: 'id',
              tablename: '',
            },
          },
          alias: '',
        },
        {
          argument: {
            field: {
              fieldname: 'name',
              tablename: '',
            },
          },
          alias: '',
        },
      ],
      limit: 10,
      // Default values.
      dryrun: false,
      tablealias: '',
      type: DpmAgentQuery.Type.DATA,
      groupbyList: [],
      orderbyList: [],
      joinsList: [],
    };

    expect(dpmQuery.toObject(false)).toEqual(want);
  });

  test('returns expected Query message for a query with selections, and filter', () => {
    const query = table
      .select('id', 'name')
      .filter(name.like('%bah%').and(createdOn.before(new Date('2023-01-01'))))
      .limit(10);
    const dpmQuery = makeDpmAgentQuery(query);
    const want: DpmAgentQuery.AsObject = {
      id: {
        packageid: 'pkg-123',
        sourceid: '',
      },
      clientversion: {
        client: ClientVersion.Client.NODE_JS,
        datasetversion: '0.1.0',
        codeversion: '0.1.0',
      },
      selectfrom: 'testTable',
      selectList: [
        {
          argument: {
            field: {
              fieldname: 'id',
              tablename: '',
            },
          },
          alias: '',
        },
        {
          argument: {
            field: {
              fieldname: 'name',
              tablename: '',
            },
          },
          alias: '',
        },
      ],
      filter: {
        op: DpmAgentQuery.BooleanExpression.BooleanOperator.AND,
        argumentsList: [
          {
            condition: {
              op: DpmAgentQuery.BooleanExpression.BooleanOperator.LIKE,
              argumentsList: [
                {
                  field: {
                    fieldname: 'name',
                    tablename: '',
                  },
                },
                {
                  literal: {
                    string: '%bah%',
                    // Unfortunately, the <message>.AsObject type does not
                    // handle oneof fields as expected; all non-object type
                    // fields are required to be defined.
                    f32: 0,
                    f64: 0,
                    i32: 0,
                    i64: 0,
                    pb_boolean: false,
                    timestamp: 0,
                    ui32: 0,
                    ui64: 0,
                  },
                },
              ],
            },
          },
          {
            condition: {
              op: DpmAgentQuery.BooleanExpression.BooleanOperator.LT,
              argumentsList: [
                {
                  field: {
                    fieldname: 'created_on',
                    tablename: '',
                  },
                },
                {
                  literal: {
                    string: '2023-01-01',
                    f32: 0,
                    f64: 0,
                    i32: 0,
                    i64: 0,
                    pb_boolean: false,
                    timestamp: 0,
                    ui32: 0,
                    ui64: 0,
                  },
                },
              ],
            },
          },
        ],
      },
      limit: 10,
      // Default values.
      dryrun: false,
      tablealias: '',
      type: DpmAgentQuery.Type.DATA,
      groupbyList: [],
      orderbyList: [],
      joinsList: [],
    };

    expect(dpmQuery.toObject(false)).toEqual(want);
  });

  test('returns expected Query message for a query with selections, filter, aggregations', () => {
    const query = table
      .select('id', 'name', price.avg().as('avgPrice'))
      .filter(name.like('%bah%').and(createdOn.before(new Date('2023-01-01'))))
      .orderBy(['avgPrice', 'DESC'], [createdOn, 'ASC']) // Note that createdOn is not in the seletion.
      .limit(10);
    const dpmQuery = makeDpmAgentQuery(query);
    const want: DpmAgentQuery.AsObject = {
      id: {
        packageid: 'pkg-123',
        sourceid: '',
      },
      clientversion: {
        client: ClientVersion.Client.NODE_JS,
        datasetversion: '0.1.0',
        codeversion: '0.1.0',
      },
      selectfrom: 'testTable',
      selectList: [
        {
          argument: {
            field: {
              fieldname: 'id',
              tablename: '',
            },
          },
          alias: '',
        },
        {
          argument: {
            field: {
              fieldname: 'name',
              tablename: '',
            },
          },
          alias: '',
        },
        {
          argument: {
            aggregate: {
              op: DpmAgentQuery.AggregateExpression.AggregateOperator.MEAN,
              argument: {
                field: {
                  fieldname: 'price',
                  tablename: '',
                },
              },
            },
          },
          alias: 'avgPrice',
        },
      ],
      filter: {
        op: DpmAgentQuery.BooleanExpression.BooleanOperator.AND,
        argumentsList: [
          {
            condition: {
              op: DpmAgentQuery.BooleanExpression.BooleanOperator.LIKE,
              argumentsList: [
                {
                  field: {
                    fieldname: 'name',
                    tablename: '',
                  },
                },
                {
                  literal: {
                    string: '%bah%',
                    // Unfortunately, the <message>.AsObject type does not
                    // handle oneof fields as expected; all non-object type
                    // fields are required to be defined.
                    f32: 0,
                    f64: 0,
                    i32: 0,
                    i64: 0,
                    pb_boolean: false,
                    timestamp: 0,
                    ui32: 0,
                    ui64: 0,
                  },
                },
              ],
            },
          },
          {
            condition: {
              op: DpmAgentQuery.BooleanExpression.BooleanOperator.LT,
              argumentsList: [
                {
                  field: {
                    fieldname: 'created_on',
                    tablename: '',
                  },
                },
                {
                  literal: {
                    string: '2023-01-01',
                    f32: 0,
                    f64: 0,
                    i32: 0,
                    i64: 0,
                    pb_boolean: false,
                    timestamp: 0,
                    ui32: 0,
                    ui64: 0,
                  },
                },
              ],
            },
          },
        ],
      },
      groupbyList: [
        {
          field: {
            fieldname: 'id',
            tablename: '',
          },
        },
        {
          field: {
            fieldname: 'name',
            tablename: '',
          },
        },
        {
          field: {
            fieldname: 'created_on',
            tablename: '',
          },
        },
      ],
      orderbyList: [
        {
          argument: {
            aggregate: {
              op: DpmAgentQuery.AggregateExpression.AggregateOperator.MEAN,
              argument: {
                field: {
                  fieldname: 'price',
                  tablename: '',
                },
              },
            },
          },
          direction: DpmAgentQuery.OrderByExpression.Direction.DESC,
        },
        {
          argument: {
            field: {
              fieldname: 'created_on',
              tablename: '',
            },
          },
          direction: DpmAgentQuery.OrderByExpression.Direction.ASC,
        },
      ],
      limit: 10,
      // Default values.
      dryrun: false,
      tablealias: '',
      type: DpmAgentQuery.Type.DATA,
      joinsList: [],
    };

    expect(dpmQuery.toObject(false)).toEqual(want);
  });
});
