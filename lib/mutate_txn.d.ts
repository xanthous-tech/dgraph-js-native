import { MutateTxn, Mutation } from '../native';
export declare class MutateTransaction {
    private txn;
    constructor(txn: MutateTxn);
    mutate(mutation: Mutation): Promise<{
        [key: string]: string;
    }>;
    commit(): Promise<void>;
}
