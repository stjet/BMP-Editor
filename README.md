# BMP Editor
BMP Editor is an [online](https://bmp.prussia.dev) .BMP image file editor that allows creation and editing of .BMP image files.

This was made using Yew, a super cool framework that compiles into Web Assembly so it can be run on the web. All the BMP reading and manipulation is done using the [bmp-rust](https://github.com/jetstream0/BMP-Rust) crate, which I also wrote.

## Features
- Create a new .BMP file, or a load an existing one.
- See the file drawn onto a canvas.
- Click canvas to see coordinate and color of the pixel, and change the color if you want.
- Use the paintbucket or click to change color tool.
- Draw lines, ellipses, and rectangles.
- Useful filters: gaussian blur, greyscaling, inverting, and more.
- Useful keyboard shortcuts for all tools, and even use the `[` and `]` to cycle through the tools.
- Do `ctrl+z` to undo actions.
- Works offline!
- When finished, name and download the .BMP file.

## Running Locally
First install Rust, and [Trunk](https://trunkrs.dev/) (trunk downloads `wasm-bindgen` and stuff like that for you).

```bash
trunk serve
```

Now visit the site at http://localhost:8080/. You can change which port it runs on by editing the `Trunk.toml` file. Even once you stop the Trunk server, the site should still be visitable and usable since it registers a service worker that allows it to work offline.
