use neon::prelude::*;

use std::sync::{Arc, Mutex};

use crate::js::client::JsDgraphClient;
use crate::js::mutation::JsMutation;
use crate::classes::MutatedTxnWrapper;
use crate::tasks::{MutateTask, CommitTask};

declare_types! {
  pub class JsMutatedTxn for MutatedTxnWrapper {
    init(mut ctx) {
      let client = ctx.argument::<JsDgraphClient>(0)?;
      let guard = ctx.lock();
      let client = client.borrow(&guard);

      Ok(MutatedTxnWrapper { txn: Arc::new(Mutex::new(Some(client.new_mutated_txn()))) })
    }

    method mutate(mut ctx) {
      let mutation = ctx.argument::<JsMutation>(0)?;
      let cb = ctx.argument::<JsFunction>(1)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn);
      let mu = mutation.borrow(&guard).clone();

      let task = MutateTask {
        txn,
        mu,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }

    method commit(mut ctx) {
      let cb = ctx.argument::<JsFunction>(0)?;

      let this = ctx.this();
      let guard = ctx.lock();

      let txn = this.borrow(&guard).txn.clone();
      Arc::downgrade(&txn);

      let task = CommitTask {
        txn,
      };

      task.schedule(cb);

      Ok(ctx.undefined().upcast())
    }
  }
}
