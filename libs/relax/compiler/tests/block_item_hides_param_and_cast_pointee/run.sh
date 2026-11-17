#!/bin/bash
# usage: run.sh <relax binary>. `relax test` of this crate must run its four tests: a body block's
# item hides a type parameter of the same name (`use crate::inner::Flags as F;` in
# `fn alias_hides_param<T, F: FnMut(u8) -> u8>`, and `struct U;` in `fn block_struct_hides_param`),
# a reference-to-reference cast decides a pointee a method left open (`v.as_ref() as &[u8]`), and
# core 1.70's `default_iters` (`impl<I: Default> Default for Cloned<I>`).
# The shapes, from `relax test sandsea-api`'s dependencies, before the fixes:
#   E0599: no function or associated item named `IS_SORTED_ANY` found for type parameter `T2`
#          (polars-core-0.55.2/src/chunked_array/ops/arity.rs:590)
#   E0283: type annotations needed: cannot satisfy `Vec<u8>: AsRef<_>`
#          (datafusion-functions-aggregate-55.2.0/src/min_max/min_max_bytes.rs:262)
#   E0277: the trait bound `Cloned<Iter<Sorted>>: Default` is not satisfied
#          (polars-plan-0.55.2/src/plans/optimizer/sortedness.rs:188)
# rustc runs the same tests: `cargo test` in this directory.
BIN="$1"
H=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$H/target"
cd "$H" || exit 1
"$BIN" test block_item_hides_param_and_cast_pointee > "$H/target/run.log" 2>&1
st=$?
if grep -qE 'E0599: no function or associated item named .IS_SORTED_ANY|E0283|E0277' "$H/target/run.log"; then
  echo "block_item_hides_param_and_cast_pointee: a block item did not hide a parameter, a cast did not decide a pointee, or an adapter has no Default"
  grep -E 'E0599|E0283|E0277' "$H/target/run.log" | head -4
  exit 1
fi
if ! grep -q 'test result: ok. 4 passed' "$H/target/run.log"; then
  echo "block_item_hides_param_and_cast_pointee: FAILED (status $st)"
  tail -20 "$H/target/run.log"
  exit 1
fi
echo "block_item_hides_param_and_cast_pointee: ok"
