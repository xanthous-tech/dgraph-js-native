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
const errors_1 = require("./errors");
class Txn {
    constructor(txn) {
        this.txn = txn;
        this.responses = {};
        this.finished = false;
        this.startPolling();
    }
    loop() {
        if (this.finished) {
            return;
        }
        this.txn.poll((err, event) => {
            if (err) {
                if (err.message.indexOf('Poll Timeout Error') > -1) {
                    this.startPolling();
                    return;
                }
                console.error(err);
                this.startPolling();
                return;
            }
            if (this.responses[event.id]) {
                this.responses[event.id][0](event.response);
                delete this.responses[event.id];
            }
            this.startPolling();
        });
    }
    startPolling() {
        if (!this.finished) {
            this.immediate = setImmediate(this.loop.bind(this));
        }
        else {
            clearImmediate(this.immediate);
        }
    }
    query(query) {
        return __awaiter(this, void 0, void 0, function* () {
            return new Promise((resolve, reject) => {
                const id = this.txn.query(query);
                this.responses[id] = [resolve, reject];
            });
        });
    }
    queryWithVars(query, vars) {
        return __awaiter(this, void 0, void 0, function* () {
            return new Promise((resolve, reject) => {
                const id = this.txn.queryWithVars(query, vars);
                this.responses[id] = [resolve, reject];
            });
        });
    }
    mutate(mutation) {
        return __awaiter(this, void 0, void 0, function* () {
            const txn = this.txn;
            if (this.isMutated(txn)) {
                return new Promise((resolve, reject) => {
                    const id = txn.mutate(mutation);
                    this.responses[id] = [resolve, reject];
                });
            }
            else {
                return Promise.reject(errors_1.READ_ONLY_TXN);
            }
        });
    }
    commit() {
        return __awaiter(this, void 0, void 0, function* () {
            const txn = this.txn;
            if (this.isMutated(txn)) {
                return new Promise((resolve, reject) => {
                    const id = txn.commit();
                    this.responses[id] = [resolve, reject];
                }).then((response) => {
                    this.finished = true;
                    return response;
                });
            }
            else {
                return Promise.reject(errors_1.READ_ONLY_TXN);
            }
        });
    }
    discard() {
        return __awaiter(this, void 0, void 0, function* () {
            const txn = this.txn;
            if (this.isMutated(txn)) {
                return new Promise((resolve, reject) => {
                    const id = txn.discard();
                    this.responses[id] = [resolve, reject];
                }).then((response) => {
                    this.finished = true;
                    return response;
                });
            }
            else {
                return Promise.reject(errors_1.READ_ONLY_TXN);
            }
        });
    }
    isMutated(txn) {
        return (typeof txn.mutate === 'function' &&
            typeof txn.commit === 'function' &&
            typeof txn.discard === 'function');
    }
}
exports.Txn = Txn;
//# sourceMappingURL=txn.js.map