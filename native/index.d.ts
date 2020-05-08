declare namespace DgraphJsNative {
  export class Client {
    constructor(servers: string[]);
    queryWithVars(query: string, vars: { [key: string]: string }): any;
  }
}

export = DgraphJsNative;
