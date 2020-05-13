"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
class Txn {
    constructor(txn) {
        this.txn = txn;
    }
    query(query) {
        return __awaiter(this, void 0, void 0, function* () {
            return new Promise((resolve, reject) => {
                this.txn.query(query, (err, result) => {
                    if (err) {
                        reject(err);
                        return;
                    }
                    resolve(result);
                });
            });
        });
    }
    queryWithVars(query, vars) {
        return __awaiter(this, void 0, void 0, function* () {
            return new Promise((resolve, reject) => {
                this.txn.queryWithVars(query, vars, (err, result) => {
                    if (err) {
                        reject(err);
                        return;
                    }
                    resolve(result);
                });
            });
        });
    }
    mutate(mutation) {
        return __awaiter(this, void 0, void 0, function* () {
            const txn = this.txn;
            if (this.isMutated(txn)) {
                return new Promise((resolve, reject) => {
                    txn.mutate(mutation, (err, result) => {
                        if (err) {
                            reject(err);
                            return;
                        }
                        resolve(result);
                    });
                });
            }
            else {
                return Promise.reject(new Error('txn is read-only'));
            }
        });
    }
    commit() {
        return __awaiter(this, void 0, void 0, function* () {
            const txn = this.txn;
            if (this.isMutated(txn)) {
                return new Promise((resolve, reject) => {
                    txn.commit((err) => {
                        if (err) {
                            reject(err);
                            return;
                        }
                        resolve();
                    });
                });
            }
            else {
                return Promise.reject(new Error('txn is read-only'));
            }
        });
    }
    discard() {
        return __awaiter(this, void 0, void 0, function* () {
            const txn = this.txn;
            if (this.isMutated(txn)) {
                return new Promise((resolve, reject) => {
                    txn.discard((err) => {
                        if (err) {
                            reject(err);
                            return;
                        }
                        resolve();
                    });
                });
            }
            else {
                return Promise.reject(new Error('txn is read-only'));
            }
        });
    }
    isMutated(txn) {
        return typeof txn.mutate === 'function'
            && typeof txn.commit === 'function'
            && typeof txn.discard === 'function';
    }
}
exports.Txn = Txn;
//# sourceMappingURL=txn.js.map