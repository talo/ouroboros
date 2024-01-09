use ouroboros::{Lambda, A, B};
use ouroboros_wasm::Callable;

#[ouroboros::entrypoint]
fn fold((f, z, xs): (Lambda<(A, B), A>, A, Vec<B>)) -> A {
    xs.into_iter().fold(z, |z, x| f.call((z, x)))
}
