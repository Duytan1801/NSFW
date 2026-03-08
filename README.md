# NSFW - Portable Text Content Filter

A high-performance, self-contained Rust CLI tool for text classification and cleaning. This tool is designed to identify and filter NSFW (Not Safe For Work) text content using ONNX models.

## Features

-   **🚀 Truly Portable**: The binary embeds the ONNX Runtime library and all necessary models. No external dependencies are required on the host system.
-   **🧹 Text Cleaning**: Automatically replaces inappropriate words with asterisks or filters them out.
-   **📊 Real-time Scoring**: Provides a classification score for the "safety" of the input text.
-   **💻 Interactive Mode**: Simple CLI interface for testing strings on the fly.
-   **⚡ High Performance**: Powered by the `ort` (ONNX Runtime) engine for fast inference.

## How It Works

This application is built for ease of distribution:
1.  **Embedding**: All models and the `libonnxruntime.so` library are compiled directly into the executable.
2.  **Auto-Extraction**: On startup, the tool extracts its runtime environment to a temporary directory (`/tmp/nsfw_bundle_v0.1.0`).
3.  **No Setup**: It "just works" out of the box on Linux systems without manual library installation or path configuration.

## Installation

### Prerequisites

-   **Rust**: You need the Rust toolchain installed (Edition 2024).

### Building from Source

```bash
git clone https://github.com/Duytan1801/NSFW.git
cd NSFW
cargo build --release
```

The portable binary will be located at `target/release/NSFW`.

## Usage

Run the binary directly:

```bash
./target/release/NSFW
```

### Modes

-   **Automated Tests**: On startup, the program runs internal tests on sample strings.
-   **Interactive Mode**: You can type any text into the prompt to see its classification score and the cleaned output. Press `Ctrl+C` to exit.

## Project Structure

-   `src/main.rs`: The core logic, including the self-extraction and model loading.
-   `models/`: Contains the ONNX models and tokenizer configurations.
-   `libonnxruntime.so`: The shared library required for model inference.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

-   Built using the [openlb](https://crates.io/crates/openlb) crate.
-   Inference powered by [ONNX Runtime](https://onnxruntime.ai/).
