# gain-knob

An example VST3/CLAP plugin demonstrating how to use [nih-plug-slint](../nih-plug-slint) to build a native Slint GUI for a [NIH-plug](https://github.com/robbert-vdh/nih-plug) audio plugin.

<img width="303" height="394" alt="image" src="https://github.com/user-attachments/assets/23a90509-d178-441c-a2ef-c1b67dcd3f66" />

## What it does

A single-knob gain plugin (-60 dB to +6 dB) with a clean, GPU-accelerated UI built entirely in Slint.

## Building

```sh
cargo build --release
cargo xtask bundle gain_knob --release
```

The bundled VST3/CLAP will be placed in `target/bundled/`.

## Project structure

```
gain-knob/
├── src/
│   ├── lib.rs          # Plugin + DSP logic
│   └── gui/
│       ├── mod.rs      # Slint module include
│       ├── ui.slint    # Main window layout
│       └── DSL/
│           └── knob.slint  # Reusable knob component
├── build.rs            # Compiles Slint UI
└── xtask/              # NIH-plug bundler
```

## Using nih-plug-slint

See [nih-plug-slint](../nih-plug-slint) for the full API reference and documentation.
