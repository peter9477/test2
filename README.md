# Compiler bug

This repo reproduces a compiler bug that initially began affecting our
code with the 2023-02-05 nightly.  Earlier nightlies did not have the
problem.

Upon shrinking the code as much as possible, the problem now occurs
with much older Rust versions.

Use this to run:

    cargo run --bin test_extract

The assertion will fail if the bug has occurred:

    user@host:~/temp/test2 $ cargo run --bin test_extract
        Updating crates.io index
       Compiling test2 v0.1.0
        Finished dev [optimized + debuginfo] target(s) in 1m 50s
         Running `probe-run --chip nrf52840 target/thumbv7em-none-eabihf/debug/test_extract`
    (HOST) INFO  flashing program (3 pages / 12.00 KiB)
    (HOST) INFO  success!
    ────────────────────────────────────────────────────────────────────────────────
    running
    └─ test_extract::__cortex_m_rt_main @ src/bin/test_extract.rs:9
    ERROR panicked at 'assertion failed: `(left == right)`
      left: `(1, 3, [3, 99])`,
     right: `(1, 99, [3, 99])`', src/bin/test_extract.rs:12:5
    └─ panic_probe::print_defmt::print @ /home/user/.cargo/registry/src/github.com-1285ae84e5963aae/panic-probe-0.3.0/src/lib.rs:91
    ────────────────────────────────────────────────────────────────────────────────
    stack backtrace:
       0: HardFaultTrampoline
