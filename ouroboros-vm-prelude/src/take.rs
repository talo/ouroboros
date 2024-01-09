use ouroboros::A;

#[ouroboros::entrypoint]
pub fn take((n, xs): (u32, Vec<A>)) -> Vec<A> {
    xs.into_iter().take(n as usize).collect::<Vec<_>>()
}
