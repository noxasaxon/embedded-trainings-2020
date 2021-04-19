# Panicking

✅ Open the `src/bin/panic.rs` file and click the "Run" button.

This program attempts to index an array beyond its length and this results in a panic.

``` console
         ERROR panicked at 'index out of bounds: the len is 3 but the index is 3', src/bin/panic.rs:30:13
└─ panic_probe::print_defmt::print @ /Users/ferrous/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.2.0/src/lib.rs:94
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
   1: lib::inline::__udf
        at ./asm/inline.rs:172
   2: __udf
        at ./asm/lib.rs:49
   3: _ZN8cortex_m3asm3udf17hd294ebe72bad7ac6E.47
   4: rust_begin_unwind
        at /Users/ferrous/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-probe-0.2.0/src/lib.rs:75
   5: core::panicking::panic_fmt
        at /rustc/cb75ad5db02783e8b0222fee363c5f63f7e2cf5b/library/core/src/panicking.rs:92
   6: core::panicking::panic_bounds_check
        at /rustc/cb75ad5db02783e8b0222fee363c5f63f7e2cf5b/library/core/src/panicking.rs:69
   7: panic::bar
        at src/bin/panic.rs:30
   8: panic::foo
        at src/bin/panic.rs:23
   9: panic::__cortex_m_rt_main
        at src/bin/panic.rs:13
  10: main
        at src/bin/panic.rs:9
  11: ResetTrampoline
        at /Users/ferrous/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:547
  12: Reset
        at /Users/ferrous/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:550
```

In `no_std` programs the behavior of panic is defined using the `#[panic_handler]` attribute.
Our example ode imports `beginner/apps/src/lib.rs`, which uses the `panic_probe` crate for panic handling.
If we want to, we can also implement it manually:

✅ In `beginner/apps/src/lib.rs`, comment out the `panic_probe` import and `#[defmt::panic_handler]` function and add your own handler to the example:

``` diff
-use panic_probe as _;

-// same panicking *behavior* as `panic-probe` but doesn't print a panic message
-// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
-#[defmt::panic_handler]
+#[panic_handler]
-fn panic() -> ! {
+fn panic(info: &core::panic::PanicInfo) -> ! {
- cortex_m::asm::udf()
+    defmt::error!("oh no! {:?}", defmt::Debug2Format(info));
+    loop {
+        cortex_m::asm::bkpt()
+    }
+}
```

Now run the program again. Try changing the format string of the `error!` macro.
