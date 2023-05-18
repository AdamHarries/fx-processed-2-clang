# fx-processed-2-clang

fx-processed-2-clang is a proof-of-concept tool to transform Firefox's processed profile(s) into profiles compatible with Clang's PGO (profile guided optimisation) format.

## Rationale

Clang's PGO is a (potentially) valuable tool for the squeezing out the "final 1%" of performance in the Firefox codebase. However, to get the most out of it, you need to have a lot of data about how and what your application performs in order to convince Clang to carry out useful optimisations. In most uses of Clang's PGO this is achieved by first building an *instrumented* build of the application, which is then run to create a profile. Instrumentation, however, presents a large runtime cost, meaning this approach is only useful local builds, and *definitely* not suitable for publishing. Because of this, runs of Firefox using instrumentation are somewhat artificial, as they will never be run by "real" users, and definitely not en-masse.

A lower-overhead alternative to instrumented compilation is the application of a *sampling profiler* to a normally-compiled run. Sampling profilers present a compromise: By sampling (instead of instrumenting), the amount of "extra code" run is small, resulting in a lower overhead, but the number of samples is smaller, and so are *statistical* rather than *complete*.

If we wanted to use *sampling profiles* in Clang's PGO, how would we go about it? Well, it turns out that Firefox has a sampling profiler built-in! It comes in the form of the [Gecko Profiler](profiler.firefox.com), and as part of the profiling process, profiles are uploaded to Mozilla-controlled servers to enable sharing and later analysis.

This collection of user-generated profiles presents an attractive source of information about how Firefox performs *in the real world*, and could be an excellent dataset to feed into Clang's PGO. However, the format that the Firefox Profiler emits is vastly different to what Clang's PGO expects. This project attempts to bridge that gap, parsing a Firefox generated profile, and emitting a Clang-compatible profile.

## Current status

This project is currently on hold, as it turns out there are a number of significant difficult problems to solve as part of this:
- We need to be able to support symbol fetching for multiple platforms. This is non-trivial, especially when we need to look up symbols from other locations.
- We need to be able to distinguish between samples taken in the "body" of a function, and in the "prologue". This is a major driver of clang's PGO inlining heuristic (as the body/prologue ratio determines how much it's been called vs how much work it does), so is important to implement.

## TODO:

- [ ] Document build process
- [ ] Add symbol fetching, a-la https://gist.github.com/luser/193572147c401c8a965c or https://hg.mozilla.org/users/jwatt_jwatt.org/fetch-symbols/file/tip/fetch-symbols.py
- [ ] Add support for debug information to symbolication step
- [ ] Implement PGO format writer