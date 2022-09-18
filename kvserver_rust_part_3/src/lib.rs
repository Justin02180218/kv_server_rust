mod config;
pub use config::*;

mod pb;
pub use pb::cmd::*;

mod args;
pub use args::*;

mod storage;
pub use storage::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
