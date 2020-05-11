import { MutateTxn, Mutation } from '../native';

export class MutateTransaction {
  private txn: MutateTxn;

  constructor(txn: MutateTxn) {
    this.txn = txn;
  }

  public async mutate(mutation: Mutation): Promise<{ [key: string]: string }> {
    return new Promise((resolve, reject) => {
      this.txn.mutate(mutation, (err, result) => {
        if (err) {
          reject(err);
          return;
        }

        resolve(result);
      });
    });
  }

  public async commit(): Promise<void> {
    return new Promise((resolve, reject) => {
      this.txn.commit((err) => {
        if (err) {
          reject(err);
          return;
        }

        resolve();
      });
    });
  }
}
