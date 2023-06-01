import { Table } from '../table';

// Defines the execution backend interface.
export interface Backend {
  compile(query: Table): Promise<string>;
  execute<Row>(query: Table): Promise<Row[]>;
}
