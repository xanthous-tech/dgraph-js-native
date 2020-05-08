import debug from 'debug';

const log = debug('dgraph-js-native:index');
import { Client } from '../native';

async function main() {
  log('creating client');

  const client = new Client(['http://localhost:9080']);
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

  client.queryWithVars(query, vars, (err, result) => {
    if (err) {
      log(err);
      return;
    }

    log(result);
  });

  // log(client.queryWithVars(query, vars));
}

main();
