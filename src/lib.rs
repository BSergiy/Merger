extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate filesize;
extern crate threadpool;

pub mod configuration;
pub mod merger;

pub use configuration::conf::Conf;
pub use merger::Merger;