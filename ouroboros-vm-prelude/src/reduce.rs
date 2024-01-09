use ouroboros::{Lambda, A};
use ouroboros_wasm::Callable;

#[ouroboros::entrypoint]
fn reduce((f, xs): (Lambda<(A, A), A>, Vec<A>)) -> Option<A> {
    match xs.len() {
        0 => None,
        1 => xs.into_iter().next(),
        _ => {
            let mut xs = xs.into_iter();
            let z = xs.next().unwrap();
            Some(xs.fold(z, |z, x| f.call((z, x))))
        }
    }
}
