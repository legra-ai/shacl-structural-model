//! SHACL Core constraint components.

mod closedness;
mod declaration;
mod display;
mod kind_name;
mod named_or_list;

pub use closedness::Closedness;
pub use declaration::Component;
pub use named_or_list::NamedOrList;
