use ouroboros::{Lambda, A, B, C};
use ouroboros_wasm::Callable;

#[ouroboros::entrypoint]
pub fn compose((f, g, x): (Lambda<B, C>, Lambda<A, B>, A)) -> C {
    f.call(g.call(x))
}
