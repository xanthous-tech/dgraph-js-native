import { QueryTxn, Mutation, Response } from '../native';
export declare type TxnOptions = {
    readOnly?: boolean;
    bestEffort?: boolean;
};
export declare class Txn {
    private txn;
    private responses;
    private finished;
    private immediate;
    constructor(txn: QueryTxn);
    private loop;
    private startPolling;
    query(query: string): Promise<Response>;
    queryWithVars(query: string, vars: {
        [key: string]: string;
    }): Promise<Response>;
    mutate(mutation: Mutation): Promise<Response>;
    commit(): Promise<Response>;
    discard(): Promise<Response>;
    private isMutated;
}
