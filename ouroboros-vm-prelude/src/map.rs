use ouroboros::{Lambda, A, B};
use ouroboros_wasm::Callable;

#[ouroboros::entrypoint]
fn map((f, xs): (Lambda<A, B>, Vec<A>)) -> Vec<B> {
    xs.into_iter().map(|x| f.call(x)).collect::<Vec<_>>()
}
