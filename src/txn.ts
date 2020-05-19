import debug from 'debug';
import { QueryTxn, MutateTxn, Mutation, ResponseEvent, Response as NativeResponse } from '../native';
import { Response } from './response';
import { READ_ONLY_TXN, ALREADY_FINISHED } from './errors';

const log = debug('dgraph-js-native:txn');

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
        // only TryRecvError here
        this.startPolling();
        return;
      }

      if (this.responses[event.id]) {
        if (event.error) {
          if (event.error.indexOf('EmptyTxn') < 0) {
            this.responses[event.id][1](new Error(event.error));
          } else {
            log('empty txn, swallowing');
            this.responses[event.id][0](new Response(new NativeResponse(JSON.stringify({ empty: true }), {})));
          }
        } else {
          if (event.response) {
            this.responses[event.id][0](new Response(event.response));
          }
        }

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
    log('query', query);

    await this.checkIsFinished();

    return new Promise((resolve, reject) => {
      const id = this.txn.query(query);
      this.responses[id] = [resolve, reject];
    });
  }

  public async queryWithVars(query: string, vars: { [key: string]: any } = {}): Promise<Response> {
    log('queryWithVars', query, vars);

    await this.checkIsFinished();

    return new Promise((resolve, reject) => {
      const id = this.txn.queryWithVars(query, vars);
      this.responses[id] = [resolve, reject];
    });
  }

  public async mutate(mutation: Mutation): Promise<Response> {
    log('mutate', mutation);

    await this.checkIsFinished();

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

  public async upsert(query: string, mutation: Mutation, commitNow?: boolean): Promise<Response> {
    log('upsert', query, mutation);

    await this.checkIsFinished();

    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        const id = commitNow ? txn.upsertAndCommitNow(query, mutation) : txn.upsert(query, mutation);
        this.responses[id] = [resolve, reject];
      });
    } else {
      return Promise.reject(READ_ONLY_TXN);
    }
  }

  public async upsertWithVars(query: string, mutation: Mutation, vars: { [key: string]: any } = {}, commitNow?: boolean): Promise<Response> {
    log('upsertWithVars', query, mutation, vars);

    await this.checkIsFinished();

    const txn = this.txn;
    if (this.isMutated(txn)) {
      return new Promise((resolve, reject) => {
        const id = commitNow ? txn.upsertWithVarsAndCommitNow(query, vars, mutation) : txn.upsertWithVars(query, vars, mutation);
        this.responses[id] = [resolve, reject];
      });
    } else {
      return Promise.reject(READ_ONLY_TXN);
    }
  }

  public async commit(): Promise<Response> {
    log('commit');

    await this.checkIsFinished();

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
    log('discard');

    await this.checkIsFinished();

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

  private checkIsFinished(): Promise<void> {
    if (this.finished) {
      return Promise.reject(ALREADY_FINISHED);
    }
    return Promise.resolve();
  }

  private isMutated(txn: QueryTxn | MutateTxn): txn is MutateTxn {
    return (
      typeof (txn as MutateTxn).mutate === 'function' &&
      typeof (txn as MutateTxn).commit === 'function' &&
      typeof (txn as MutateTxn).discard === 'function'
    );
  }
}
