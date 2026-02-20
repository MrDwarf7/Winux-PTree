pub mod cache;
// pub mod cache_lazy;
// pub mod cache_limcode;
// pub mod cache_mmap;
// pub mod cache_opt;
pub mod cache_rkyv;

// TODO: [errors] : We want to ideally remove
// anyhow! from a lib crate, and do the same as the others -
// move over to using `thiserror = { workspace = true }` and
// move the anyhow usage to map_err(...) calls.

pub use cache::{
    DirEntry,
    DiskCache,
    USNJournalState,
    compute_content_hash,
    get_cache_path,
    get_cache_path_custom,
    has_directory_changed,
};
