// utils_mod.rs

//! various utilities

/// Initialize tracing to file logs/github_readme_copy.log.  \
///
/// The folder logs/ is in .gitignore and will not be committed.  
pub fn tracing_init() -> anyhow::Result<()> {
    let offset = time::UtcOffset::current_local_offset()?;
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"),
    );

    // A filter consists of one or more comma-separated directives
    // target[span{field=value}]=level
    // Levels order: 1. ERROR, 2. WARN, 3. INFO, 4. DEBUG, 5. TRACE
    // ERROR level is always logged.
    // Add filters to GITHUB_README_COPY_LOG environment variable for a single execution:
    // ```bash
    // GITHUB_README_COPY_LOG="debug,hyper_util=info,reqwest=info" ./{package_name}
    // ```
    let filter = tracing_subscriber::EnvFilter::from_env("GITHUB_README_COPY_LOG");

    let builder = tracing_subscriber::fmt()
        .with_timer(timer)
        .with_ansi(false)
        .with_target(false)
        .with_env_filter(filter);
    if std::env::var("GITHUB_README_COPY_LOG").is_ok() {
        // if GITHUB_README_COPY_LOG exists than enable tracing to file
        let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("github_readme_copy")
            .filename_suffix("log")
            .build("logs")
            .expect("initializing rolling file appender failed");
        builder.with_writer(file_appender).init();
    } else {
        builder.init();
    };

    Ok(())
}

/// macro to get source code position to log errors before propagation
///
/// example:  read_to_string("x").log(pos!())?;
#[macro_export]
macro_rules! pos {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        &format!("{}:{}:{}:", file!(), line!(), column!())
    };
}

/// Trait to log the error from Result before propagation with ?.
pub trait ResultLogError<T, E>: Sized {
    fn log(self, file_line_column: &str) -> Self;
}

/// Implements LogError for anyhow::Result.
impl<T, E: std::fmt::Debug> ResultLogError<T, E> for core::result::Result<T, E> {
    fn log(self, file_line_column: &str) -> Self {
        self.inspect_err(|err| tracing::error!("{} {:?}", file_line_column, err))
    }
}

// region: delimiters cannot be INACTIVE like markers

/// return the position of start of the delimited data after the delimiter
pub fn find_pos_start_data_after_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Option<usize> {
    if let Some(pos_start_data) = find_from(md_text_content, pos, delimiter) {
        let pos_start_data = pos_start_data + delimiter.len();
        return Some(pos_start_data);
    }
    // return
    None
}

/// return the position of end of the delimited data before the delimiter
pub fn find_pos_end_data_before_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Option<usize> {
    if let Some(pos_end_data) = find_from(md_text_content, pos, delimiter) {
        return Some(pos_end_data);
    }
    //return
    None
}

// endregion: delimiters cannot be INACTIVE like markers

/// find from_pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Option<usize> {
    let slice01 = text.get(from_pos..).expect("find_from out of index");
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        // return Option with usize
        Some(from_pos + location)
    } else {
        // return Option with none
        option_location
    }
}
