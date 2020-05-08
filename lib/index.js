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
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const debug_1 = __importDefault(require("debug"));
const log = debug_1.default("dgraph-js-native:index");
const native_1 = require("../native");
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        log('creating client');
        const client = new native_1.Client(['http://localhost:9080']);
        const query = `
  query user($userId: string) {
    var(func: uid($userId)) {
      has_user_group @filter(NOT eq(removed, true)) {
        userGroupUid as uid
        has_workspace @facets(hasPermission: CAN_WORKSPACE_READ) @facets(eq(CAN_WORKSPACE_READ, true)) {
          permitted_has_workspace as uid
        }
        has_core @facets(hasPermission: CAN_CORE_READ) @facets(eq(CAN_CORE_READ, true)) {
          permitted_has_core as uid
        }
        has_table @facets(hasPermission: CAN_TABLE_READ) @facets(eq(CAN_TABLE_READ, true)) {
          permitted_has_table as uid
        }
        has_view @facets(hasPermission: CAN_VIEW_READ) @facets(eq(CAN_VIEW_READ, true)) {
          permitted_has_view as uid
        }
      }
    }
    userGroups(func: uid(userGroupUid)) {
      uid
      removed
    }
    workspaces(func: uid(permitted_has_workspace)) {
      uid
    }
    cores(func: uid(permitted_has_core)) {
      uid
    }
    tables(func: uid(permitted_has_table)) {
      uid
    }
    views(func: uid(permitted_has_view)) {
      uid
    }
  }
  `;
        const vars = { $userId: '0x1' };
        log(client.queryWithVars(query, vars));
    });
}
main();
//# sourceMappingURL=index.js.map