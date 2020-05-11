import { QueryTxn } from '../native';
export declare class QueryTransaction {
    private txn;
    constructor(txn: QueryTxn);
    query(query: string): Promise<any>;
    queryWithVars(query: string, vars: {
        [key: string]: string;
    }): Promise<any>;
}
