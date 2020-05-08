declare namespace DgraphJsNative {
  export class Client {
    constructor(servers: string[]);
    queryWithVars(query: string, vars: { [key: string]: string }, cb: (err: any, result: any) => void): void;
  }
}

export = DgraphJsNative;
