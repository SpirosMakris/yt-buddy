use crate::{DocumentLoader, LoaderError};
use async_trait::async_trait;
use llm_chain::schema::Document;
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum YoutubeCaptionsLoaderError {
    #[error("Generic error")]
    GeneralError,
    #[error("Reqwest error: {0}")]
    FetchHtmlError(reqwest::Error),
    #[error("Captions JSON extract error")]
    ExtractCaptionsJsonError,
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

        let result = html
            .split_once(captions_separator)
            .ok_or(YoutubeCaptionsLoaderError::ExtractCaptionsJsonError)?
            .1;

        let result = result
            .split_once(video_details_separator)
            .ok_or(YoutubeCaptionsLoaderError::ExtractCaptionsJsonError)?
            .0
            .to_string();

        let value: serde_json::Value = serde_json::from_str(&result).unwrap();

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

pub type YoutubeCaptionsLoaderMetadata = Vec<(String, String)>;

#[async_trait]
impl DocumentLoader<YoutubeCaptionsLoaderMetadata> for YoutubeCaptionsLoader {
    async fn load(&self) -> Result<Vec<Document<YoutubeCaptionsLoaderMetadata>>, LoaderError> {
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

        let metadata = vec![
            ("video_id".to_string(), transcript.video_id.clone()),
            (
                "language_code".to_string(),
                transcript.language_code.clone(),
            ),
            (
                "translation_langs".to_string(),
                transcript.translation_langs.join(","),
            ),
            (
                "is_generated".to_string(),
                transcript.is_generated.to_string(),
            ),
        ];

        Ok(vec![Document {
            page_content: transcript_strs.join(" "),
            metadata: Some(metadata),
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
