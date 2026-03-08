use openlb::text_filter::TxtCleaner;
use std::fs;
use std::path::PathBuf;
use std::env;

// Embedding the ONNX Runtime library
const LIB_ORT: &[u8] = include_bytes!("../libonnxruntime.so");

// Embedding the models
const MODEL_ONNX: &[u8] = include_bytes!("../models/text_classify.onnx");
const MODEL_DATA: &[u8] = include_bytes!("../models/text_classify.onnx.data");
const MODEL_TOKENIZER: &[u8] = include_bytes!("../models/text_tokenizer.json");
const CONFIG_JSON: &[u8] = include_bytes!("../models/config.json");
const SPECIAL_TOKENS_MAP: &[u8] = include_bytes!("../models/special_tokens_map.json");
const TOKENIZER_CONFIG: &[u8] = include_bytes!("../models/tokenizer_config.json");
const TOKENIZER_JSON: &[u8] = include_bytes!("../models/tokenizer.json");
const VOCAB_TXT: &[u8] = include_bytes!("../models/vocab.txt");

fn setup_runtime() -> anyhow::Result<PathBuf> {
    // We use a stable directory in /tmp for the runtime files
    let runtime_dir = PathBuf::from("/tmp/nsfw_bundle_v0.1.0");
    let models_dir = runtime_dir.join("models");

    fs::create_dir_all(&models_dir)?;

    // Extract ONNX Runtime library
    let lib_path = runtime_dir.join("libonnxruntime.so");
    if !lib_path.exists() {
        fs::write(&lib_path, LIB_ORT)?;
    }
    
    // Set the ORT_DYLIB_PATH environment variable so the 'ort' crate can find it
    unsafe {
        env::set_var("ORT_DYLIB_PATH", &lib_path);
    }

    // Helper to write model files if they are missing
    let files = [
        ("text_classify.onnx", MODEL_ONNX),
        ("text_classify.onnx.data", MODEL_DATA),
        ("text_tokenizer.json", MODEL_TOKENIZER),
        ("config.json", CONFIG_JSON),
        ("special_tokens_map.json", SPECIAL_TOKENS_MAP),
        ("tokenizer_config.json", TOKENIZER_CONFIG),
        ("tokenizer.json", TOKENIZER_JSON),
        ("vocab.txt", VOCAB_TXT),
    ];

    for (name, bytes) in files.iter() {
        let path = models_dir.join(name);
        if !path.exists() {
            fs::write(path, bytes)?;
        }
    }

    Ok(runtime_dir)
}

fn main() -> anyhow::Result<()> {
    // 1. Extract embedded models and runtime library to a temporary location
    println!("Initializing runtime...");
    let runtime_dir = setup_runtime()?;

    // 2. Save current directory and switch to runtime directory
    // This is because openlb (v0.1) hardcodes looking for './models/' in the current directory.
    let original_dir = env::current_dir()?;
    env::set_current_dir(&runtime_dir)?;

    // 3. Initialize the TxtCleaner (loads models into memory)
    println!("Loading models into memory (this may take a moment)...");
    let txtcleaner = TxtCleaner::builder().commit();

    // 4. Restore original directory so the user can use relative paths if they want
    env::set_current_dir(original_dir)?;
    println!("System ready.\n");

    let test_texts = vec![
        "This is a completely safe and normal message about weather and daily activities.",
        "Here are some explicit adult content that should be flagged as inappropriate.",
    ];

    for text in test_texts {
        let classification = txtcleaner.classify_text(text);
        let cleaned = txtcleaner.clean_text(text);

        println!("Original: {}", text);
        println!("Classification: score={:.4}", classification[0][0]);
        println!("Cleaned: {}\n", cleaned);
    }

    println!("Enter text to classify (Ctrl+C to quit):");
    let mut input = String::new();
    while std::io::stdin().read_line(&mut input)? > 0 {
        let text = input.trim();
        if !text.is_empty() {
            let classification = txtcleaner.classify_text(text);
            let cleaned = txtcleaner.clean_text(text);
            println!("score: {:.4}", classification[0][0]);
            println!("cleaned: {}\n", cleaned);
        }
        input.clear();
    }

    Ok(())
}
