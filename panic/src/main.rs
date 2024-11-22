fn main() {
    // If an error occurs in a Rust programme, it will panic. This may be as
    // the result of trying to access data from outside of the bounds of an
    // array, or we can use the panic! macro to trigger it ourselves.

    // panic!("crash and burn");

    // Note the contents of cargo.toml. By default Rust will clean up any
    // memory used by the programme itself, in a process called unwinding.
    // However, this can take time. The alternative is to simply abort the
    // programme, and leave it to the OS to free up the memory.

    // Consider a simple scenario which would cause Rust to panic
    let v = vec![1, 2, 3];
    v[99];

    // Using cargo run gives a brief description of the error here, showing
    // the index out of bounds message

    // Using `RUST_BACKTRACE=1 cargo run` gives a full traceback. This
    // behaviour is only enabled when running without the --release flag, as
    // debug symbols incur a performance penalty
}
