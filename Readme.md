## Repro for bindgen issue with Windows DLLs

To run the reproducer, from a VS x64 Developer Command Prompt, execute `run.cmd`.
`cargo build` will fail with a linker error.

To resolve the linker error, add `#[link(name = "example")]` to the `extern` block in the generated `bindings.rs` file.
