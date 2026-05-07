pub mod domain;
pub mod outbound;

#[cfg(test)]
pub mod tests;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
