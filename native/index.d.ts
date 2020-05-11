declare namespace DgraphJsNative {
  export class Mutation {
    setSetJson(jsonString: string): void;
    setSetNquads(nquadsString: string): void;
    setDeleteJson(deleteJsonString: string): void;
    setDelNquads(delNquadsString: string): void;
  }

  export interface QueryTxn {
    query(query: string, cb: (err: any, result: any) => void): void;
    queryWithVars(query: string, vars: { [key: string]: string }, cb: (err: any, result: any) => void): void;
  }

  export interface MutateTxn {
    mutate(mutation: Mutation, cb: (err: any, result: any) => void): void;
    commit(cb: (err: any) => void): void;
  }

  export class Client {
    constructor(servers: string[]);
    newQueryTxn(isBestEffort: boolean): QueryTxn;
    newMutateTxn(): MutateTxn;
  }

  export class ReadOnlyTxn implements QueryTxn {}
  export class BestEffortTxn implements QueryTxn {}
  export class MutatedTxn implements MutateTxn {}
}

export = DgraphJsNative;
