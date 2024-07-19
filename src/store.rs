use std::path::Path;
use oxigraph::store::Store as Dataset;
pub use oxigraph::store::StorageError;

/// [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset) store based on
/// [RocksDB](https://rocksdb.org/).
#[allow(dead_code)] // TODO: Remove `#[allow(dead_code)]` once the `dataset` field is used.
pub struct Store {
    dataset: Dataset,
}

impl Store {
    /// Initialize a [`Store`].
    fn init(dataset: Dataset) -> Result<Self, StorageError> {
	Ok(Self {
	    dataset,
	})
    }

    /// Create a temporary [`Store`].
    pub fn new() -> Result<Self, StorageError> {
	Self::init(Dataset::new()?)
    }

    /// Open a persistent [`Store`], or create it if it doesn't exist.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
	Self::init(Dataset::open(path)?)
    }
}
