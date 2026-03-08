# NSFW - Portable Text Content Filter

A self-contained CLI tool for classifying and cleaning NSFW text content.

## 🚀 Quick Start (Linux)

Download the pre-built binary and run it:

```bash
wget https://github.com/Duytan1801/NSFW/releases/download/v0.1.0/NSFW
chmod +x NSFW
./NSFW
```

## 🛠️ Build from Source

```bash
git clone https://github.com/Duytan1801/NSFW.git
cd NSFW
cargo build --release
# Binary is at: target/release/NSFW
```

## Features
- **Zero Dependencies**: Everything (models + runtime) is embedded in one binary.
- **Fast**: Powered by ONNX Runtime for high-performance inference.
- **Interactive**: Simple interface to get safety scores and cleaned text.
<img width="838" height="497" alt="image" src="https://github.com/user-attachments/assets/42877965-9827-4587-90cf-d65f565eafd6" />

---
Built with [openlb](https://crates.io/crates/openlb).
