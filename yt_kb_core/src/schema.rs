use std::borrow::Cow;

#[derive(Debug)]
#[deprecated(note = "Please use llm-chain::schema::Document for compatibility")]
pub struct CowDocument<'a, M = EmptyMetadata> {
    pub page_content: Cow<'a, str>,
    pub metadata: Option<M>,
}

impl<'a, M> CowDocument<'a, M> {
    pub fn new<T: Into<Cow<'a, str>>>(content: T) -> Self {
        Self {
            page_content: content.into(),
            metadata: None,
        }
    }

    pub fn metadata(&self) -> Option<&M> {
        self.metadata.as_ref()
    }
}

#[derive(Debug)]
pub struct EmptyMetadata;
