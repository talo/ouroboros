use std::ffi::{CStr, CString};

#[no_mangle]
pub fn echo(v: *mut u8) -> *mut i8 {
    let json_str = unsafe { CStr::from_ptr(v as *const i8) };
    let json = serde_json::from_str::<serde_json::Value>(json_str.to_str().expect("invalid utf8"))
        .expect("invalid json");
    let json = match json {
        serde_json::Value::Object(mut map) => {
            map.insert(
                "hello".to_string(),
                serde_json::Value::String("world!".to_string()),
            );
            serde_json::Value::Object(map)
        }
        _ => json,
    };
    let json_str = serde_json::to_string(&json).expect("invalid json");
    CString::new(json_str).unwrap().into_raw()
}
