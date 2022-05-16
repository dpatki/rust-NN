# rust-NN
Rust NN project based on https://pwy.io/en/posts/learning-to-fly-pt1/

## Technology Stack

A Rust Backend is compiled to WebAssembly and served to users with a simple JS Frontend.
Uses a canvas rendering context to draw the graphics to the screen.

## Demo
![Demo Video](https://user-images.githubusercontent.com/64248134/168679346-e67863eb-22a1-497e-a8c7-db6c354ea833.gif)

## Instructions
Compile the Rust backend by running `wasm-pack build --target web` in the `simulation-wasm` directory. Then run npm start in the `www` directory.
