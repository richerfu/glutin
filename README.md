# glutin -  OpenGL, UTilities, and INput

A low-level library for OpenGL context creation.

[![](https://img.shields.io/crates/v/glutin.svg)](https://crates.io/crates/glutin)
[![Docs.rs](https://docs.rs/glutin/badge.svg)](https://docs.rs/glutin)

## [Documentation](https://docs.rs/glutin)

## Contact Us

Join us in any of these:

[![Matrix](https://img.shields.io/badge/Matrix-%23winit%3Amatrix.org-blueviolet.svg)](https://matrix.to/#/#winit:matrix.org)
[![Libera.Chat](https://img.shields.io/badge/libera.chat-%23winit-red.svg)](https://web.libera.chat/#winit)

## Usage Examples

**Warning:** These are examples for `master`. You can find examples for
the latest _released version_ [here](https://github.com/rust-windowing/glutin/releases/latest).

The examples use [`gl_generator`](https://crates.io/crates/gl_generator) to
generate OpenGL bindings.

### Try it!

```bash
git clone https://github.com/rust-windowing/glutin
cd glutin
cargo run --example window
```

### Usage

Glutin is an OpenGL context creation library, and doesn't directly provide
OpenGL bindings for you.

For examples, please look [here](https://github.com/rust-windowing/glutin/tree/master/glutin_examples).

Note that glutin aims at being a low-level brick in your rendering
infrastructure. You are encouraged to write another layer of abstraction
between glutin and your application.

Glutin follows winit's [MSRV policy](https://github.com/rust-windowing/winit/blob/master/README.md#msrv-policy).

## Platform-specific notes

### Android

Be sure to handle Android's lifecycle correctly when using a `winit` window
by only creating a GL surface after `winit` raises `Event::Resumed`, and
destroy it again upon receiving `Event::Suspended`. See this in action in the
[`android.rs` example](./glutin_examples/examples/android.rs).

To compile and run the Android example on your device,
install [`cargo-apk`](https://crates.io/crates/cargo-apk)
and start the app using:

```console
$ cargo apk r -p glutin_examples --example android
```

### OpenHarmony / HarmonyOS NEXT

The existing [`ohos.rs` example](./glutin_examples/examples/ohos.rs) uses the
Ability-backed winit 0.31 event loop. Create the GL surface in
`ApplicationHandler::can_create_surfaces` and release it in `destroy_surfaces`.
On OpenHarmony, `run_app` retains the owned application and returns to the
Ability UI thread; the previous `spawn_app` entry point is no longer needed.

Set `OHOS_NDK_HOME` to the OpenHarmony SDK directory containing `native/`,
install [`ohrs`](https://crates.io/crates/ohrs), then build the native example:

```console
$ ohrs build --arch arm64 -p glutin_examples -- --example ohos --no-default-features --features egl
```

The packaged output is `dist/arm64-v8a/libohos.so`. This is the native module;
package it in an ArkTS application using the same Ability revision and its
matching HARs. The example does not assemble or install a HAP.
The workspace pins the updated OHOS backend and requires Rust 1.88 for OHOS.
