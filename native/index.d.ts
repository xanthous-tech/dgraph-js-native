declare namespace DgraphJsNative {
  export interface ResponseEvent {
    id: string;
    response?: Response;
    error?: string;
  }

  export class Response {
    constructor(json: string, uidsMap: { [key: string]: string });
    getJson(): any;
    getUidsMap(): { [key: string]: string };
  }

  export class Operation {
    setSchema(schema: string): void;
  }

  export class Mutation {
    clearSetList(): void;
    setSetJson(jsonString: string): void;
    setSetNquads(nquadsString: string): void;
    setDeleteJson(deleteJsonString: string): void;
    setDelNquads(delNquadsString: string): void;
  }

  export interface QueryTxn {
    query(query: string): string;
    queryWithVars(query: string, vars: { [key: string]: any }): string;
    poll(cb: (err: Error, resp: ResponseEvent) => void): void;
  }

  export interface MutateTxn extends QueryTxn {
    upsert(query: string, mutation: Mutation): string;
    upsertAndCommitNow(query: string, mutation: Mutation): string;
    upsertWithVars(query: string, vars: { [key: string]: any }, mutation: Mutation): string;
    upsertWithVarsAndCommitNow(query: string, vars: { [key: string]: any }, mutation: Mutation): string;
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
