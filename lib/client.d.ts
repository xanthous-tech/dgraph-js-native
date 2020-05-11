import { Client } from '../native';
import { QueryTransaction } from './query_txn';
import { MutateTransaction } from './mutate_txn';
export declare class DgraphClient extends Client {
    newQueryTransaction(isBestEffort?: boolean): QueryTransaction;
    newMutateTransaction(): MutateTransaction;
}
