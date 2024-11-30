use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Commands {
    /// Output file path or "-" for stdout
    #[arg(short = 'o', long = "output", default_value = "output.txt.zst")]
    pub output: String,

    /// Chunk size for parallel processing
    #[arg(short = 's', long = "chunk-size", default_value = "1000000")]
    pub chunk_size: usize,

    /// Number of zstd compression threads (0 or 1 to disable multithreading)
    #[arg(short = 't', long = "zstd-threads", default_value = "8")]
    pub zstd_threads: u32,

    /// Size of the bounded channel queue
    #[arg(short = 'q', long = "queue-size", default_value = "1024")]
    pub queue_size: usize,
}
