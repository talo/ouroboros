use ouroboros::{A, B};

#[ouroboros::entrypoint]
fn swap((x, y): (A, B)) -> (B, A) {
    (y, x)
}
