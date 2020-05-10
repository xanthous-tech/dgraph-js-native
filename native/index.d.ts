declare namespace DgraphJsNative {
  export class Client {
    constructor(servers: string[]);
    newQueryTxn(isBestEffort: boolean): QueryTxn;
  }

  export interface QueryTxn {
    query(query: string, cb: (err: any, result: any) => void): void;
    queryWithVars(query: string, vars: { [key: string]: string }, cb: (err: any, result: any) => void): void;
  }

  export class ReadOnlyTxn implements QueryTxn {}
  export class BestEffortTxn implements QueryTxn {}
}

export = DgraphJsNative;
