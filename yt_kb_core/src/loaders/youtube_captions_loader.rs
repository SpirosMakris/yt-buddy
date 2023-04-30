use crate::{DocumentLoader, LoaderError};
use async_trait::async_trait;
use llm_chain::schema::Document;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YoutubeCaptionsLoaderError {
    #[error("Generic error")]
    GeneralError,
}

pub struct YoutubeCaptionsLoader {
    video_id: String,
}

impl YoutubeCaptionsLoader {
    pub fn new(video_id: String) -> Self {
        Self { video_id }
    }

    async fn fetch_html(&self) -> Result<String, reqwest::Error> {
        let url = self.create_url(&self.video_id);
        let body = reqwest::get(url).await?.text().await?;
        let text = html_escape::decode_html_entities(&body);

        Ok(text.into_owned())
    }

    async fn extract_captions_json(
        &self,
        html: String,
    ) -> Result<CaptionsList, YoutubeCaptionsLoaderError> {
        let captions_separator = r#""captions":"#;
        let video_details_separator = r#","videoDetails"#;

        let result = html.split(captions_separator).skip(1).collect::<String>();

        let result = result
            .split(video_details_separator)
            .next()
            .unwrap()
            .to_string();

        let value: serde_json::Value = serde_json::from_str(&result).unwrap();
        // println!("3: {deserialized:?}");
        let captions_list: CaptionsList =
            serde_json::from_value(value["playerCaptionsTracklistRenderer"].clone()).unwrap();

        println!("3: {captions_list:?}");

        Ok(captions_list)
    }

    fn create_url(&self, video_id: &str) -> String {
        format!("https://www.youtube.com/watch?v={video_id}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CaptionsList {
    caption_tracks: Option<Vec<CaptionTrack>>,
    translation_languages: Option<Vec<TranslationLanguage>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslationLanguage {
    language_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CaptionTrack {
    base_url: String,
    language_code: String,
    is_translatable: bool,
    kind: String,
}

#[derive(Debug)]
struct Transcript {
    video_id: String,
    url: String,
    language_code: String,
    is_generated: bool,
    translation_langs: Vec<String>,
}

impl Transcript {
    fn from_captions_list(
        captions_list: CaptionsList,
        video_id: String,
    ) -> Result<Vec<Transcript>, LoaderError> {
        if let Some(tracks) = captions_list.caption_tracks {
            let res = tracks
                .iter()
                .map(|t| Transcript {
                    video_id: video_id.clone(),
                    language_code: t.language_code.clone(),
                    translation_langs: vec![], // @TODO: add translation langs here
                    is_generated: t.kind == "asr",
                    url: t.base_url.clone(),
                })
                .collect();

            Ok(res)
        } else {
            Ok(vec![])
        }
    }

    pub async fn fetch(&self) -> Result<Vec<String>, reqwest::Error> {
        let res = reqwest::get(self.url.clone()).await?.text().await.unwrap();
        let doc = roxmltree::Document::parse(&res).unwrap();

        let mut transcript: Vec<String> = Vec::new();

        let nodes = doc.descendants().filter(|x| x.tag_name() == "text".into());

        for node in nodes {
            let text = html_escape::decode_html_entities(node.text().unwrap());
            transcript.push(text.into());
        }

        Ok(transcript)
    }
}

#[async_trait]
impl DocumentLoader for YoutubeCaptionsLoader {
    type Metadata = HashMap<String, String>;

    async fn load(&self) -> Result<Vec<Document<Self::Metadata>>, LoaderError> {
        let consent_str = r#"action="https://consent.youtube.com/s""#;

        let html_str = self.fetch_html().await.map_err(|_e| {
            LoaderError::SourceReadError(format!("Failed to fetch HTML for {}", self.video_id))
        })?;

        if html_str.contains(consent_str) {
            panic!("Needs consent cookie.fix");
        }
        // @TODO: Perform checks if splitting fails(in combinator chain)
        // @TODO: Check validity of video-id
        // @TODO: Check if recaptcha and error out
        // @TODO: Check for playbality status

        let caps = self.extract_captions_json(html_str).await.unwrap();
        let transcripts = Transcript::from_captions_list(caps, self.video_id.clone())?;

        let transcript = transcripts.get(0).unwrap();

        let transcript_strs = transcript.fetch().await.unwrap();

        Ok(vec![Document {
            page_content: transcript_strs.join(" "),
            metadata: None,
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn load_1_test() {
        let video_id = "XZtlD_m59sM";

        let loader = YoutubeCaptionsLoader::new(video_id.to_string());
        let res = loader.load().await.unwrap();

        println!("res: {res:?}");
    }
}
