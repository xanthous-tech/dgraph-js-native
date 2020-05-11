import { QueryTxn } from '../native';

export class QueryTransaction {
  private txn: QueryTxn;

  constructor(txn: QueryTxn) {
    this.txn = txn;
  }

  public async query(query: string): Promise<any> {
    return new Promise((resolve, reject) => {
      this.txn.query(query, (err, result) => {
        if (err) {
          reject(err);
          return;
        }

        resolve(result);
      });
    });
  }

  public async queryWithVars(query: string, vars: { [key: string]: string }): Promise<any> {
    return new Promise((resolve, reject) => {
      this.txn.queryWithVars(query, vars, (err, result) => {
        if (err) {
          reject(err);
          return;
        }

        resolve(result);
      });
    });
  }
}
