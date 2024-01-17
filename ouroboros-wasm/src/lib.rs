use std::{
    ffi::{c_char, CStr, CString},
    mem::ManuallyDrop,
};

use ouroboros::{Lambda, Type, TypeInfo};
use serde::{Deserialize, Serialize};

#[repr(u8)]
pub enum ErrorCode {
    Success = 0,
    InvalidJson = 1,
    InvalidUtf8 = 2,
    MemoryOutOfBounds = 3,
    Internal = 255,
}

#[derive(Deserialize, Serialize)]
pub struct Manifest<'a> {
    pub name: &'a str,
    pub input: Type,
    pub output: Type,
}

pub enum ParseResult<'a, I> {
    Manifest(Manifest<'a>),
    Args(I),
}

/// # Safety
/// It is assumed that `args` is a pointer returned by `__ouroboros__alloc` that
/// points to a nul-terminated C string.
pub unsafe fn decode_args<I, O>(name: &str, args: *const c_char) -> ParseResult<I>
where
    I: Deserialize<'static> + TypeInfo,
    O: TypeInfo,
{
    let args = unsafe { CStr::from_ptr(args) }
        .to_str()
        .expect("invalid utf8");

    if args
        .split(' ')
        .next()
        .map(|s| s.trim().starts_with("--manifest"))
        .unwrap_or(false)
    {
        ParseResult::Manifest(Manifest {
            name,
            input: I::t(),
            output: O::t(),
        })
    } else {
        ParseResult::Args(serde_json::from_str(args).expect("invalid json"))
    }
}

pub fn encode_result<Result>(result: Result) -> *mut c_char
where
    Result: serde::Serialize,
{
    CString::new(serde_json::to_string(&result).expect("invalid json"))
        .unwrap()
        .into_raw()
}

pub fn encode_result_pretty<Result>(result: Result) -> *mut c_char
where
    Result: serde::Serialize,
{
    CString::new(serde_json::to_string_pretty(&result).expect("invalid json"))
        .unwrap()
        .into_raw()
}

pub trait Callable {
    type Args: Serialize;
    type Ret: Deserialize<'static>;

    fn call(&self, args: Self::Args) -> Self::Ret;
}

impl<A, B> Callable for Lambda<A, B>
where
    A: Serialize,
    B: Deserialize<'static>,
{
    type Args = A;
    type Ret = B;

    fn call(&self, args: Self::Args) -> Self::Ret {
        use std::{ptr, slice};

        let lambda = serde_json::to_vec(&self).expect("invalid lambda");
        let args = serde_json::to_vec(&args).expect("invalid args");

        let mut ret = ptr::null();
        let mut ret_size = 0u32;

        let mut err_code = 0u32;

        unsafe {
            __ouroboros__call_fn(
                lambda.as_ptr(),
                lambda.len() as u32,
                args.as_ptr(),
                args.len() as u32,
                &mut ret,
                &mut ret_size,
                &mut err_code,
            );
        }

        if err_code != 0 {
            panic!("`__ouroboros__call_fn` returned error code {}", err_code);
        }

        serde_json::from_slice(unsafe { slice::from_raw_parts(ret, ret_size as usize) })
            .expect("invalid fn result")
    }
}

#[no_mangle]
pub extern "C" fn __ouroboros__alloc(size: usize) -> *mut u8 {
    let mut buf = ManuallyDrop::new(Vec::with_capacity(size));
    buf.as_mut_ptr()
}

/// # Safety
/// It is assumed that `ptr` is a pointer returned by `__ouroboros__alloc`. Not
/// other pointer is valid.
#[no_mangle]
pub unsafe extern "C" fn __ouroboros__free(ptr: *mut u8, size: usize) {
    let mut buf = ManuallyDrop::new(unsafe { Vec::from_raw_parts(ptr, size, size) });
    ManuallyDrop::drop(&mut buf);
}

extern "C" {
    pub fn __ouroboros__call_fn(
        lambda_ptr: *const u8,
        lambda_size: u32,

        args_ptr: *const u8,
        args_size: u32,

        ret_ptr: *mut *const u8,
        ret_size: *mut u32,

        err_code_ptr: *mut u32,
    );

    pub fn __ouroboros__call_mod(
        module_ptr: *const u8,
        module_size: u32,

        args_ptr: *const u8,
        args_size: u32,

        ret_ptr: *mut *const u8,
        ret_size: *mut u32,

        err_code_ptr: *mut u32,
    );
}
