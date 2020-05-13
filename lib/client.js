"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const debug_1 = __importDefault(require("debug"));
const native_1 = require("../native");
const txn_1 = require("./txn");
const errors_1 = require("./errors");
const log = debug_1.default('dgraph-js-native:client');
class DgraphClient extends native_1.Client {
    newTxn(options) {
        if (!options) {
            return new txn_1.Txn(super.newMutateTxn());
        }
        if (options.readOnly) {
            return new txn_1.Txn(super.newQueryTxn(options.bestEffort || false));
        }
        else {
            if (options.bestEffort) {
                log(`Client attempted to query using best-effort without setting the transaction to read-only`);
                throw errors_1.ERR_BEST_EFFORT_REQUIRED_READ_ONLY;
            }
            else {
                return new txn_1.Txn(super.newMutateTxn());
            }
        }
    }
}
exports.DgraphClient = DgraphClient;
//# sourceMappingURL=client.js.map