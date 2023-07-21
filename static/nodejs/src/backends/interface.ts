import { Table } from '../table';

/**
 * The execution backend interface.
 */
export interface Backend {
  compile(query: Table): Promise<string>;
  execute<Row extends object>(query: Table): Promise<Row[]>;
}
