use ouroboros::A;

#[ouroboros::entrypoint]
pub fn skip((n, xs): (u32, Vec<A>)) -> Vec<A> {
    xs.into_iter().skip(n as usize).collect::<Vec<_>>()
}
