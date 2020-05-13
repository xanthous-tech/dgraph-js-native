import { QueryTxn, MutateTxn, Mutation, Response } from '../native';
import { READ_ONLY_TXN } from './errors';

export type TxnOptions = {
  readOnly?: boolean;
  bestEffort?: boolean;
};

export class Txn {
  private txn: QueryTxn | MutateTxn;

  constructor(txn: QueryTxn) {
    this.txn = txn;
  }

  public async query(query: string): Promise<Response> {
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

  public async queryWithVars(query: string, vars: { [key: string]: string }): Promise<Response> {
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

  public async mutate(mutation: Mutation): Promise<Response> {
    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        txn.mutate(mutation, (err, result) => {
          if (err) {
            reject(err);
            return;
          }

          resolve(result);
        });
      });
    } else {
      return Promise.reject(READ_ONLY_TXN);
    }
  }

  public async commit(): Promise<void> {
    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        txn.commit((err) => {
          if (err) {
            reject(err);
            return;
          }

          resolve();
        });
      });
    } else {
      return Promise.reject(READ_ONLY_TXN);
    }
  }

  public async discard(): Promise<void> {
    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        txn.discard((err) => {
          if (err) {
            reject(err);
            return;
          }

          resolve();
        });
      });
    } else {
      return Promise.reject(READ_ONLY_TXN);
    }
  }

  private isMutated(txn: QueryTxn | MutateTxn): txn is MutateTxn {
    return (
      typeof (txn as MutateTxn).mutate === 'function' &&
      typeof (txn as MutateTxn).commit === 'function' &&
      typeof (txn as MutateTxn).discard === 'function'
    );
  }
}
