# MyLLM
A toy experiment in using an LLM in Rust.

Uses LLM model `llama-2-7b-chat.ggmlv3.q2_K.bin` found on [huggingface.com](https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML/resolve/main/) by HuggingFace user, TheBloke.

Uses the latest main branch(commit id: `9376078c12ea1990bd42e63432656819a056d379`) of Rust's `llm` crate as of 2023-12-19.

TO build this, I needed to use an [example provided in the repo](https://github.com/rustformers/llm/blob/main/crates/llm/examples/inference.rs), modifying the example provided by the `llm` crate's [official documentation on crates.io](https://github.com/rustformers/llm).

https://isaacwyatt.com