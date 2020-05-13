import { QueryTxn, Mutation, Response } from '../native';
export declare type TxnOptions = {
    readOnly?: boolean;
    bestEffort?: boolean;
};
export declare class Txn {
    private txn;
    constructor(txn: QueryTxn);
    query(query: string): Promise<Response>;
    queryWithVars(query: string, vars: {
        [key: string]: string;
    }): Promise<Response>;
    mutate(mutation: Mutation): Promise<Response>;
    commit(): Promise<void>;
    discard(): Promise<void>;
    private isMutated;
}
