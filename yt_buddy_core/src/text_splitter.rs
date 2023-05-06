use std::sync::{Arc, Mutex};

use llm_chain::{tokens::Tokenizer, TextSplitter};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

use iter_tools::Itertools;

const MAX_SEQ_LENGTH: usize = 128;

pub struct RsBertTextSplitter {
    model: Arc<Mutex<SentenceEmbeddingsModel>>,
}

impl RsBertTextSplitter {
    pub fn new(model: Arc<Mutex<SentenceEmbeddingsModel>>) -> Self {
        Self { model }
    }
}

type TokenType = i64;

impl Tokenizer<TokenType> for RsBertTextSplitter {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<TokenType>, llm_chain::tokens::TokenizerError> {
        // Get tokenizer
        let model = self
            .model
            .lock()
            .map_err(|_e| llm_chain::tokens::TokenizerError::TokenizerCreationError)?;

        // The tokenizer has a max_seq_lenqth property. It truncates after the limit so we
        // split the text into and tokenize each chunk seperately.
        let chunked_doc = doc
            .split_whitespace()
            .chunks(MAX_SEQ_LENGTH)
            .into_iter()
            .map(|c| c.into_iter().join(" "))
            .collect::<Vec<String>>();

        let mut tokenized: Vec<TokenType> = vec![];

        for chunk in chunked_doc.iter() {
            let tokenized_chunk = model
                .tokenize(&[chunk])
                .tokens_ids
                .into_iter()
                .flat_map(|t| {
                    let vec: Vec<i64> = t.into();
                    vec.into_iter()
                });

            tokenized.extend(tokenized_chunk);
        }

        println!("Tokenized: {tokenized:?}");
        println!("Tokenized length: {}", tokenized.len());

        Ok(tokenized)
    }

    fn to_string(
        &self,
        tokens: Vec<TokenType>,
    ) -> Result<String, llm_chain::tokens::TokenizerError> {
        let model = self
            .model
            .lock()
            .map_err(|_e| llm_chain::tokens::TokenizerError::TokenizationError)?;

        let tokenizer = model.get_tokenizer();
        let tokens = tokens.into_iter().collect::<Vec<i64>>();

        // @TODO: Check out `skip_special_tokens` & `clean_up_tokenization_spaces` attributes
        // What should their values be?
        Ok(tokenizer.decode(&tokens, true, true))
    }
}

impl TextSplitter<TokenType> for RsBertTextSplitter {}
