use llm::{Model, models::Llama};
use std::{io::Write, convert::Infallible};

fn main() {
    // load a GGML model from disk
    // downloaded from  https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML/resolve/main/llama-2-7b-chat.ggmlv3.q2_K.bin
    let llama : Llama = llm::load(std::path::Path::new("src/llama-2-7b-chat.ggmlv3.q2_K.bin.1"),
            llm::TokenizerSource::Embedded,
            Default::default(),
            llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    // use the model to generate text from a prompt
    let mut session = llama.start_session(Default::default());

    let res = session.infer::<Infallible>(
        &llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: "Rust is a cool programming language".into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        
        // OutputRequest
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();

                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
}
