import { Client } from '../native';
import { QueryTransaction } from './query_txn';
import { MutateTransaction } from './mutate_txn';

export class DgraphClient extends Client {
  public newQueryTransaction(isBestEffort = false): QueryTransaction {
    return new QueryTransaction(super.newQueryTxn(isBestEffort));
  }

  public newMutateTransaction(): MutateTransaction {
    return new MutateTransaction(super.newMutateTxn());
  }
}
