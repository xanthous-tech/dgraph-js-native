"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const debug_1 = __importDefault(require("debug"));
const log = debug_1.default('dgraph-js-native:index');
const native_1 = require("../native");
function main() {
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
    const queryTxn = client.newQueryTxn(false);
    queryTxn.queryWithVars(query, vars, (err, result) => {
        if (err) {
            log(err);
            return;
        }
        log(result);
    });
    const mutateTxn = client.newMutateTxn();
    const mutation = new native_1.Mutation();
    mutation.setSetNquads(`
    _:a <dgraph.type> "WORKSPACE" .
    _:a <name> "workspace from dgraph-js-native" .
  `);
    mutateTxn.mutate(mutation, (err, result) => {
        if (err) {
            log(err);
            return;
        }
        log(result);
        mutateTxn.commit((err) => {
            if (err) {
                log(err);
            }
        });
    });
}
main();
//# sourceMappingURL=index.js.map