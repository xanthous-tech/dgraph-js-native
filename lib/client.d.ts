import { Client } from '../native';
import { Txn, TxnOptions } from './txn';
export declare class DgraphClient extends Client {
    newTxn(options?: TxnOptions): Txn;
}
