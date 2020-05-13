import debug from 'debug';
import { Client } from '../native';
import { Txn, TxnOptions } from './txn';
import { ERR_BEST_EFFORT_REQUIRED_READ_ONLY } from './errors';

const log = debug('dgraph-js-native:client');

export class DgraphClient extends Client {
  public newTxn(options?: TxnOptions): Txn {
    if (!options) {
      return new Txn(super.newMutateTxn());
    }

    if (options.readOnly) {
      return new Txn(super.newQueryTxn(options.bestEffort || false));
    } else {
      if (options.bestEffort) {
        log(`Client attempted to query using best-effort without setting the transaction to read-only`);
        throw ERR_BEST_EFFORT_REQUIRED_READ_ONLY;
      } else {
        return new Txn(super.newMutateTxn());
      }
    }
  }
}
