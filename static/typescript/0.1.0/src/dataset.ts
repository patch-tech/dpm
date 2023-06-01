// Dataset: a container for tables.

import { Table } from './table';

export class Dataset {
  private tableByName: {[name: string]: Table} = {};

  constructor(
    readonly name: string,
    readonly version: string) { }

    addTable(table: Table) {
      if (table.name in this.tableByName) {
        throw Error(`Table named ${table.name} alreadu exists`);
      }
      this.tableByName[table.name] = table;
    }

    getTable(name: string): Table | undefined {
      return this.tableByName[name];
    }
}