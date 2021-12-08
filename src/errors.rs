// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
    }
}

pub static SEARCH_QUERY_MISSING: &str = "No search query";
pub static NO_SEARCH_RESULTS: &str = "No search results available";
pub static BUFFER_MISSING: &str = "No buffer available";