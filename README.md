# clocksource - high performance clocks for Rust

Clocksource allows access to alternate clocksources like the TSC on your Intel x86 CPU. Most modern processors support 'constant_tsc' allowing us to use this counter as a high resolution clock. The cost of reading this counter can be much lower than calls to 'clock_gettime()' - especially on virtualized environments.

Unfortunately, this requires we use nightly rust until the asm!() macro is stabilized. We provide fallback for users on stable rust, this should allow zero-cost abstraction of clock_gettime() for stable builds without benefit of the high-performance timing.

The API documentation of this library can be found at
[docs.rs/clocksource](https://docs.rs/clocksource/).

## Getting clocksource

add `clocksource` to your dependencies and start using it

## Future Work

* Additional platform support

## Contributing

All contributions are subject to the original LICENSE

* fork it
* feature branch
* improve it
* write a good commit message
* push to your fork
* open a PR