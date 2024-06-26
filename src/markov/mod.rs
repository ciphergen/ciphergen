mod generator;
mod model;
mod load;

pub use load::{load_corpus, load_default_corpus};
pub use generator::{MarkovGenerator, Generator};

#[allow(unused_imports)]
pub use model::{MarkovModel, Model};
