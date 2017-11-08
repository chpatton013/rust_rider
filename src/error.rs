extern crate error_chain;
extern crate serde_json;
extern crate std;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
  foreign_links {
    SerdeJson(serde_json::error::Error);
    StdFmt(std::fmt::Error);
    StdIo(std::io::Error) #[cfg(unix)];
  }
}
