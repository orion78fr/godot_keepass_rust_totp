# What

A Keepass TOTP app, made with Godot, using GDNative bindings with Rust.

# Why

**Why Godot ?** Because I wanted to train godot to make games and it's multiplatform (Windows, Linux, MacOS, Android, HTML5).

**Why Rust ?** Because I like Rust and want to learn it better. Plus it's compatible with Godot and I don't really like GDScript for "complex" things (even though in theory, it's a powerful language).

**Why a Keepass TOTP app ?** Because Keepass2Android app is great, but not for TOTP. I wanted a simple view with all my TOTP codes from my keepassxc database. They are in a separate database than my passwords so it's having a hard time to autofill anyway.

**Why not contribute to Keepass2Android ?** Cause it's C# and I don't know it. And even though I'm interested, not as much as Rust. Plus I think it's a reasonable sized project.

**Just why ? Don't you have something better to do ?** I'm post burnout with a big impostor syndrome so I need some small projects to do or I will lose all motivation to dev again.

# How

Here I will list various useful links / tips :

 - [Auto completion for Jetbrains IDEs](https://godot-rust.github.io/book/faq.html#auto-completion-with-intellij-rust-plugin)
 - I use [Github Actions](https://github.com/orion78fr/godot_keepass_rust_totp/blob/master/.github/workflows/build.yml) to compile to all targets (except iOS, cause I didn't take the time to set it up)
 - Wasm target for godot-rust was not really supported, but I made it work with various hacks. [Example (might not work, I'm using this to do tests)](https://orion78.fr/godot_test/keepassTotp.html)
 - Fixed bug in `chrono` crate related to compilation for wasm target [BUG #519](https://github.com/chronotope/chrono/issues/519) [PR #568](https://github.com/chronotope/chrono/pull/568)
 - Fixed bug in `gdnative-sys` for Android compilation on Windows [PR #754](https://github.com/godot-rust/godot-rust/pull/754)

# Contribute

Don't hesitate ! Even if it's something small / dirty, I'll be happy.

If you want to give me some money, I haven't set anything up yet, but don't hesitate to contact me in this case, we'll find a way.

Even though I'm not streaming a lot right now, you can follow me [on Twitch](https://www.twitch.tv/orion78fr).
