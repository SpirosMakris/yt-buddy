use std::sync::Mutex;

use llm_chain::{tokens::Tokenizer, TextSplitter};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

pub struct RsBertTextSplitter<'a> {
    model: &'a Mutex<SentenceEmbeddingsModel>,
}

impl<'a> RsBertTextSplitter<'a> {
    pub fn new(model: &'a Mutex<SentenceEmbeddingsModel>) -> Self {
        Self { model }
    }
}

type TokenType = i64;

impl Tokenizer<TokenType> for RsBertTextSplitter<'_> {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<TokenType>, llm_chain::tokens::TokenizerError> {
        let tokenized = self
            .model
            .lock()
            .map_err(|e| llm_chain::tokens::TokenizerError::TokenizerCreationError)?
            .tokenize(&[doc])
            .tokens_ids
            .into_iter()
            .map(|t| t.int64_value(&[]))
            .collect();

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

impl TextSplitter<TokenType> for RsBertTextSplitter<'_> {}
