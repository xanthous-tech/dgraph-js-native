"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const native_1 = require("../native");
const query_txn_1 = require("./query_txn");
const mutate_txn_1 = require("./mutate_txn");
class DgraphClient extends native_1.Client {
    newQueryTransaction(isBestEffort = false) {
        return new query_txn_1.QueryTransaction(super.newQueryTxn(isBestEffort));
    }
    newMutateTransaction() {
        return new mutate_txn_1.MutateTransaction(super.newMutateTxn());
    }
}
exports.DgraphClient = DgraphClient;
//# sourceMappingURL=client.js.map