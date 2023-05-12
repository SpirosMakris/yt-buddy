//! Simplistic Model Layer
//! (with mock-store layer)
//!
//! Goal: Flesh out the CRUD API. Later we can change the implementation
//! and the model store to get it actually working.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region: --- IngestEntry
#[derive(Clone, Debug, Serialize)]
pub struct IngestEntry {
    pub id: u64,
    pub video_id: String,
}

#[derive(Deserialize)]
pub struct IngestEntryForCreate {
    pub video_id: String,
}
// endregion: --- IngestEntry

// @NOTE: This is where an sqlx connection could reside
// This could not work in production since the vector grows indefinitely
// We derive Clone because this will be used as state. Clones Arc, not vector.
#[derive(Clone)]
pub struct ModelController {
    ingest_entries: Arc<Mutex<Vec<Option<IngestEntry>>>>,
}

// Ctor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ingest_entries: Arc::default(),
        })
    }
}

// CRUD implementation
impl ModelController {
    pub async fn create_ingest_entry(
        &self,
        ingest_entry_fc: IngestEntryForCreate,
    ) -> Result<IngestEntry> {
        let mut store = self.ingest_entries.lock().unwrap();

        let id = store.len() as u64 + 1;
        let ingest_entry = IngestEntry {
            id,
            video_id: ingest_entry_fc.video_id,
        };

        store.push(Some(ingest_entry.clone()));

        Ok(ingest_entry)
    }

    pub async fn list_ingest_entries(&self) -> Result<Vec<IngestEntry>> {
        let store = self.ingest_entries.lock().unwrap();

        // Filter maps clones the options and filters out the Nones
        let ingest_entries = store
            .iter()
            .filter_map(|entry| entry.clone())
            .collect::<Vec<_>>();

        Ok(ingest_entries)
    }

    pub async fn delete_ingest_entry(&self, id: u64) -> Result<IngestEntry> {
        let mut store = self.ingest_entries.lock().unwrap();

        let ingest_entry = store.get_mut(id as usize).and_then(|t| t.take());

        ingest_entry.ok_or(Error::IngestEntryDeleteFailIdNotFound { id })
    }

    // @TODO: Add get & update
}
