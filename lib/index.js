"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const debug_1 = __importDefault(require("debug"));
const log = debug_1.default("dgraph-js-native:index");
const addon = require("../native");
log(addon.hello());
//# sourceMappingURL=index.js.map