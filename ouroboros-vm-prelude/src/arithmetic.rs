#[ouroboros::entrypoint]
fn add_u8((x, y): (u8, u8)) -> u8 {
    x + y
}

#[ouroboros::entrypoint]
fn add_u16((x, y): (u16, u16)) -> u16 {
    x + y
}

#[ouroboros::entrypoint]
fn add_u32((x, y): (u32, u32)) -> u32 {
    x + y
}

#[ouroboros::entrypoint]
fn add_u64((x, y): (u64, u64)) -> u64 {
    x + y
}

#[ouroboros::entrypoint]
fn add_i8((x, y): (i8, i8)) -> i8 {
    x + y
}

#[ouroboros::entrypoint]
fn add_i16((x, y): (i16, i16)) -> i16 {
    x + y
}

#[ouroboros::entrypoint]
fn add_i32((x, y): (i32, i32)) -> i32 {
    x + y
}

#[ouroboros::entrypoint]
fn add_i64((x, y): (i64, i64)) -> i64 {
    x + y
}

#[ouroboros::entrypoint]
fn div_u8((x, y): (u8, u8)) -> u8 {
    x / y
}

#[ouroboros::entrypoint]
fn div_u16((x, y): (u16, u16)) -> u16 {
    x / y
}

#[ouroboros::entrypoint]
fn div_u32((x, y): (u32, u32)) -> u32 {
    x / y
}

#[ouroboros::entrypoint]
fn div_u64((x, y): (u64, u64)) -> u64 {
    x / y
}

#[ouroboros::entrypoint]
fn div_i8((x, y): (i8, i8)) -> i8 {
    x / y
}

#[ouroboros::entrypoint]
fn div_i16((x, y): (i16, i16)) -> i16 {
    x / y
}

#[ouroboros::entrypoint]
fn div_i32((x, y): (i32, i32)) -> i32 {
    x / y
}

#[ouroboros::entrypoint]
fn div_i64((x, y): (i64, i64)) -> i64 {
    x / y
}

#[ouroboros::entrypoint]
fn max_u8((x, y): (u8, u8)) -> u8 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn max_u16((x, y): (u16, u16)) -> u16 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn max_u32((x, y): (u32, u32)) -> u32 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn max_u64((x, y): (u64, u64)) -> u64 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn max_i8((x, y): (i8, i8)) -> i8 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn max_i16((x, y): (i16, i16)) -> i16 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn max_i32((x, y): (i32, i32)) -> i32 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn max_i64((x, y): (i64, i64)) -> i64 {
    x.max(y)
}

#[ouroboros::entrypoint]
fn min_u8((x, y): (u8, u8)) -> u8 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn min_u16((x, y): (u16, u16)) -> u16 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn min_u32((x, y): (u32, u32)) -> u32 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn min_u64((x, y): (u64, u64)) -> u64 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn min_i8((x, y): (i8, i8)) -> i8 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn min_i16((x, y): (i16, i16)) -> i16 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn min_i32((x, y): (i32, i32)) -> i32 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn min_i64((x, y): (i64, i64)) -> i64 {
    x.min(y)
}

#[ouroboros::entrypoint]
fn mul_u8((x, y): (u8, u8)) -> u8 {
    x * y
}

#[ouroboros::entrypoint]
fn mul_u16((x, y): (u16, u16)) -> u16 {
    x * y
}

#[ouroboros::entrypoint]
fn mul_u32((x, y): (u32, u32)) -> u32 {
    x * y
}

#[ouroboros::entrypoint]
fn mul_u64((x, y): (u64, u64)) -> u64 {
    x * y
}

#[ouroboros::entrypoint]
fn mul_i8((x, y): (i8, i8)) -> i8 {
    x * y
}

#[ouroboros::entrypoint]
fn mul_i16((x, y): (i16, i16)) -> i16 {
    x * y
}

#[ouroboros::entrypoint]
fn mul_i32((x, y): (i32, i32)) -> i32 {
    x * y
}

#[ouroboros::entrypoint]
fn mul_i64((x, y): (i64, i64)) -> i64 {
    x * y
}

#[ouroboros::entrypoint]
fn rem_u8((x, y): (u8, u8)) -> u8 {
    x % y
}

#[ouroboros::entrypoint]
fn rem_u16((x, y): (u16, u16)) -> u16 {
    x % y
}

#[ouroboros::entrypoint]
fn rem_u32((x, y): (u32, u32)) -> u32 {
    x % y
}

#[ouroboros::entrypoint]
fn rem_u64((x, y): (u64, u64)) -> u64 {
    x % y
}

#[ouroboros::entrypoint]
fn rem_i8((x, y): (i8, i8)) -> i8 {
    x % y
}

#[ouroboros::entrypoint]
fn rem_i16((x, y): (i16, i16)) -> i16 {
    x % y
}

#[ouroboros::entrypoint]
fn rem_i32((x, y): (i32, i32)) -> i32 {
    x % y
}

#[ouroboros::entrypoint]
fn rem_i64((x, y): (i64, i64)) -> i64 {
    x % y
}

#[ouroboros::entrypoint]
fn sub_u8((x, y): (u8, u8)) -> u8 {
    x - y
}

#[ouroboros::entrypoint]
fn sub_u16((x, y): (u16, u16)) -> u16 {
    x - y
}

#[ouroboros::entrypoint]
fn sub_u32((x, y): (u32, u32)) -> u32 {
    x - y
}

#[ouroboros::entrypoint]
fn sub_u64((x, y): (u64, u64)) -> u64 {
    x - y
}

#[ouroboros::entrypoint]
fn sub_i8((x, y): (i8, i8)) -> i8 {
    x - y
}

#[ouroboros::entrypoint]
fn sub_i16((x, y): (i16, i16)) -> i16 {
    x - y
}

#[ouroboros::entrypoint]
fn sub_i32((x, y): (i32, i32)) -> i32 {
    x - y
}

#[ouroboros::entrypoint]
fn sub_i64((x, y): (i64, i64)) -> i64 {
    x - y
}
