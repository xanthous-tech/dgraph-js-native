import { QueryTxn, MutateTxn, Mutation, Response, ResponseEvent } from '../native';
import { READ_ONLY_TXN } from './errors';

export type TxnOptions = {
  readOnly?: boolean;
  bestEffort?: boolean;
};

export class Txn {
  private txn: QueryTxn | MutateTxn;
  private responses: { [key: string]: [(resp: Response) => void, (err: Error) => void] };
  private finished: boolean;
  private immediate: NodeJS.Immediate;

  constructor(txn: QueryTxn) {
    this.txn = txn;
    this.responses = {};
    this.finished = false;

    this.startPolling();
  }

  private loop(): void {
    if (this.finished) {
      return;
    }

    this.txn.poll((err, event: ResponseEvent) => {
      if (err) {
        if (err.message.indexOf('Poll Timeout Error') > -1) {
          this.startPolling();
          return;
        }

        console.error(err);
        this.startPolling();
        return;
      }

      if (this.responses[event.id]) {
        this.responses[event.id][0](event.response);
        delete this.responses[event.id];
      }

      this.startPolling();
    });
  }

  private startPolling(): void {
    if (!this.finished) {
      this.immediate = setImmediate(this.loop.bind(this));
    } else {
      clearImmediate(this.immediate);
    }
  }

  public async query(query: string): Promise<Response> {
    return new Promise((resolve, reject) => {
      const id = this.txn.query(query);
      this.responses[id] = [resolve, reject];
    });
  }

  public async queryWithVars(query: string, vars: { [key: string]: string }): Promise<Response> {
    return new Promise((resolve, reject) => {
      const id = this.txn.queryWithVars(query, vars);
      this.responses[id] = [resolve, reject];
    });
  }

  public async mutate(mutation: Mutation): Promise<Response> {
    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        const id = txn.mutate(mutation);
        this.responses[id] = [resolve, reject];
      });
    } else {
      return Promise.reject(READ_ONLY_TXN);
    }
  }

  public async commit(): Promise<Response> {
    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        const id = txn.commit();
        this.responses[id] = [resolve, reject];
      }).then((response: Response) => {
        this.finished = true;
        return response;
      });
    } else {
      return Promise.reject(READ_ONLY_TXN);
    }
  }

  public async discard(): Promise<Response> {
    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        const id = txn.discard();
        this.responses[id] = [resolve, reject];
      }).then((response: Response) => {
        this.finished = true;
        return response;
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
