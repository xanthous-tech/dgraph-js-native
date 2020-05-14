declare namespace DgraphJsNative {
  export interface ResponseEvent {
    id: string;
    response: Response;
  }

  export class Response {
    getJson(): any;
    getUidsMap(): { [key: string]: string };
  }

  export class Operation {
    setSchema(schema: string): void;
  }

  export class Mutation {
    setSetJson(jsonString: string): void;
    setSetNquads(nquadsString: string): void;
    setDeleteJson(deleteJsonString: string): void;
    setDelNquads(delNquadsString: string): void;
  }

  export interface QueryTxn {
    query(query: string): string;
    queryWithVars(query: string, vars: { [key: string]: string }): string;
    poll(cb: (err: Error, resp: ResponseEvent) => void): void;
  }

  export interface MutateTxn extends QueryTxn {
    mutate(mutation: Mutation): string;
    commit(): string;
    discard(): string;
  }

  export class Client {
    constructor(servers: string[]);
    newQueryTxn(isBestEffort: boolean): QueryTxn;
    newMutateTxn(): MutateTxn;
    alter(op: Operation): string;
  }

  export class ReadOnlyTxn implements QueryTxn {}
  export class BestEffortTxn implements QueryTxn {}
  export class MutatedTxn implements MutateTxn {}
}

export = DgraphJsNative;
