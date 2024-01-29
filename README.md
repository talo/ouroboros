## `ouroboros`

This crate defines the core types and core values that are used by all of the other ouroboros crates. It also re-exports the procedural macros for convenience.

## `ouroboros-proc-macro`

Defines all the procedural macros. Ideally, these would go in a more sensisble place, but cargo does not allow that.

## `ouroboros-vm`

This is the virtual machine that allows you to run ouroboros functions.

## `ouroboros-vm-prelude`

This crate defines a set of ouroboros functions that are imported by the VM by default. They are always available.

## `ouroboros-wasm`

This crate defines the set of host functions and guest functions that make up the ouroboros interface. This includes basic guest functionality like `__ouroboros__alloc` and `__ouroboros__free` for allocating and freeing memory, as well as more complex host functions like `__ouroboros__call_fn` and `__ouroboros__call_mod`. It also defines some helpers and utilities to make interactions over the ouroboros interface simpler to do. 