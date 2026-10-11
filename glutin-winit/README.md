# glutin-winit

The crate provides cross-platform glutin `Display` bootstrapping with winit.

This crate is also a reference on how to do the bootstrapping of glutin when
used with a cross-platform windowing library.

## OpenHarmony

The workspace pins the richerfu/winit 0.31 beta facade, core, geometry crate and
Ability-backed OHOS backend together. Applications consuming this branch must
apply the same workspace-root Cargo patches. The OHOS backend requires Rust 1.88.
Use EGL on OpenHarmony; desktop X11/Wayland cfgs are excluded for `target_env = "ohos"`.
The window creation result follows this winit revision's `CreateWindowError` API.
