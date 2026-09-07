use super::{v03, v04, v08, v11, v17};
use native_db::{ToKey, native_db};
use native_model::{Model, native_model};
use serde::{Deserialize, Serialize};

pub use crate::v17::*;

// --- TORRENT ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 2, version = 18, from = v17::Torrent)]
#[native_db(export_keys = true)]
pub struct Torrent {
    #[primary_key]
    pub id: String,
    pub id_is_hash: bool,
    #[secondary_key(unique)]
    pub mam_id: u64,
    pub abs_id: Option<String>,
    pub goodreads_id: Option<u64>,
    pub library_path: Option<std::path::PathBuf>,
    pub library_files: Vec<std::path::PathBuf>,
    pub linker: Option<String>,
    pub category: Option<String>,
    pub selected_audio_format: Option<String>,
    pub selected_ebook_format: Option<String>,
    #[secondary_key]
    pub title_search: String,
    pub meta: v17::TorrentMeta,
    #[secondary_key]
    pub created_at: v03::Timestamp,
    pub replaced_with: Option<(String, v03::Timestamp)>,
    pub request_matadata_update: bool,
    pub library_mismatch: Option<v08::LibraryMismatch>,
    pub client_status: Option<v08::ClientStatus>,
}

impl From<v17::Torrent> for Torrent {
    fn from(t: v17::Torrent) -> Self {
        Self {
            id: t.id,
            id_is_hash: t.id_is_hash,
            mam_id: t.mam_id,
            abs_id: t.abs_id,
            goodreads_id: t.goodreads_id,
            library_path: t.library_path,
            library_files: t.library_files,
            linker: t.linker,
            category: t.category,
            selected_audio_format: t.selected_audio_format,
            selected_ebook_format: t.selected_ebook_format,
            title_search: t.title_search,
            meta: t.meta,
            created_at: t.created_at,
            replaced_with: t.replaced_with,
            request_matadata_update: t.request_matadata_update,
            library_mismatch: t.library_mismatch,
            client_status: t.client_status,
        }
    }
}

impl From<Torrent> for v17::Torrent {
    fn from(t: Torrent) -> Self {
        Self {
            id: t.id,
            id_is_hash: t.id_is_hash,
            mam_id: t.mam_id,
            abs_id: t.abs_id,
            goodreads_id: t.goodreads_id,
            library_path: t.library_path,
            library_files: t.library_files,
            linker: t.linker,
            category: t.category,
            selected_audio_format: t.selected_audio_format,
            selected_ebook_format: t.selected_ebook_format,
            title_search: t.title_search,
            meta: t.meta,
            created_at: t.created_at,
            replaced_with: t.replaced_with,
            request_matadata_update: t.request_matadata_update,
            library_mismatch: t.library_mismatch,
            client_status: t.client_status,
        }
    }
}

// --- SELECTED TORRENT ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 3, version = 18, from = v17::SelectedTorrent)]
#[native_db(export_keys = true)]
pub struct SelectedTorrent {
    #[primary_key]
    pub mam_id: u64,
    pub goodreads_id: Option<u64>,
    #[secondary_key(unique, optional)]
    pub hash: Option<String>,
    pub dl_link: String,
    pub unsat_buffer: Option<u64>,
    pub wedge_buffer: Option<u64>,
    pub cost: v04::TorrentCost,
    pub category: Option<String>,
    pub tags: Vec<String>,
    #[secondary_key]
    pub title_search: String,
    pub meta: v17::TorrentMeta,
    pub grabber: Option<String>,
    pub created_at: v03::Timestamp,
    pub started_at: Option<v03::Timestamp>,
    pub removed_at: Option<v03::Timestamp>,
    pub grabber_id: Option<u64>,
    pub grabber_label: Option<String>,
}

impl From<v17::SelectedTorrent> for SelectedTorrent {
    fn from(t: v17::SelectedTorrent) -> Self {
        Self {
            mam_id: t.mam_id,
            goodreads_id: t.goodreads_id,
            hash: t.hash,
            dl_link: t.dl_link,
            unsat_buffer: t.unsat_buffer,
            wedge_buffer: t.wedge_buffer,
            cost: t.cost,
            category: t.category,
            tags: t.tags,
            title_search: t.title_search,
            meta: t.meta,
            grabber: t.grabber,
            created_at: t.created_at,
            started_at: t.started_at,
            removed_at: t.removed_at,
            grabber_id: None,
            grabber_label: None,
        }
    }
}

impl From<SelectedTorrent> for v17::SelectedTorrent {
    fn from(t: SelectedTorrent) -> Self {
        Self {
            mam_id: t.mam_id,
            goodreads_id: t.goodreads_id,
            hash: t.hash,
            dl_link: t.dl_link,
            unsat_buffer: t.unsat_buffer,
            wedge_buffer: t.wedge_buffer,
            cost: t.cost,
            category: t.category,
            tags: t.tags,
            title_search: t.title_search,
            meta: t.meta,
            grabber: t.grabber,
            created_at: t.created_at,
            started_at: t.started_at,
            removed_at: t.removed_at,
        }
    }
}

// --- DUPLICATE TORRENT ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 4, version = 18, from = v17::DuplicateTorrent)]
#[native_db]
pub struct DuplicateTorrent {
    #[primary_key]
    pub mam_id: u64,
    pub dl_link: Option<String>,
    #[secondary_key]
    pub title_search: String,
    pub meta: v17::TorrentMeta,
    pub created_at: v03::Timestamp,
    pub duplicate_of: Option<String>,
}

impl From<v17::DuplicateTorrent> for DuplicateTorrent {
    fn from(t: v17::DuplicateTorrent) -> Self {
        Self {
            mam_id: t.mam_id,
            dl_link: t.dl_link,
            title_search: t.title_search,
            meta: t.meta,
            created_at: t.created_at,
            duplicate_of: t.duplicate_of,
        }
    }
}

impl From<DuplicateTorrent> for v17::DuplicateTorrent {
    fn from(t: DuplicateTorrent) -> Self {
        Self {
            mam_id: t.mam_id,
            dl_link: t.dl_link,
            title_search: t.title_search,
            meta: t.meta,
            created_at: t.created_at,
            duplicate_of: t.duplicate_of,
        }
    }
}

// --- ERRORED TORRENT ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 5, version = 18, from = v17::ErroredTorrent)]
#[native_db(export_keys = true)]
pub struct ErroredTorrent {
    #[primary_key]
    pub id: v11::ErroredTorrentId,
    pub title: String,
    pub error: String,
    pub meta: Option<v17::TorrentMeta>,
    #[secondary_key]
    pub created_at: v03::Timestamp,
}

impl From<v17::ErroredTorrent> for ErroredTorrent {
    fn from(t: v17::ErroredTorrent) -> Self {
        Self {
            id: t.id,
            title: t.title,
            error: t.error,
            meta: t.meta,
            created_at: t.created_at,
        }
    }
}

impl From<ErroredTorrent> for v17::ErroredTorrent {
    fn from(t: ErroredTorrent) -> Self {
        Self {
            id: t.id,
            title: t.title,
            error: t.error,
            meta: t.meta,
            created_at: t.created_at,
        }
    }
}

// --- EVENT ---
#[derive(Serialize, Deserialize, Debug, Clone)]
#[native_model(id = 6, version = 18, from = v17::Event)]
#[native_db(export_keys = true)]
pub struct Event {
    #[primary_key]
    pub id: v03::Uuid,
    #[secondary_key]
    pub torrent_id: Option<String>,
    #[secondary_key]
    pub mam_id: Option<u64>,
    #[secondary_key]
    pub created_at: v03::Timestamp,
    pub event: v17::EventType,
}

impl From<v17::Event> for Event {
    fn from(t: v17::Event) -> Self {
        Self {
            id: t.id,
            torrent_id: t.torrent_id,
            mam_id: t.mam_id,
            created_at: t.created_at,
            event: t.event,
        }
    }
}

impl From<Event> for v17::Event {
    fn from(t: Event) -> Self {
        Self {
            id: t.id,
            torrent_id: t.torrent_id,
            mam_id: t.mam_id,
            created_at: t.created_at,
            event: t.event,
        }
    }
}