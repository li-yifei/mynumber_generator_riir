use clap::Parser;
use commands::Commands;
use crossbeam::channel::bounded;
use rayon::iter::{IntoParallelIterator, ParallelBridge};
use rayon::prelude::ParallelIterator;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use zstd::Encoder;
mod commands;
const QN: [u8; 11] = [6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];

const fn initiate_div<const N: usize>() -> [usize; N] {
    let mut ret = [0; N];
    let mut a = 1;
    let mut i = 0;
    while i < N {
        ret[N - 1 - i] = a;
        a *= 10;
        i += 1;
    }
    ret
}
const DIV_ARR: [usize; 11] = initiate_div::<11>();
const TOTAL_ITERATIONS: usize = 100_000_000_000;

fn main() -> io::Result<()> {
    let args = Commands::parse();
    generate(args)
}

fn generate(args: Commands) -> io::Result<()> {
    let writer: Box<dyn Write + Send + Sync + 'static> = if args.output == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(&args.output)?)
    };

    let file_buf = BufWriter::with_capacity(64 * 1024 * 1024, writer);
    let mut encoder = Encoder::new(file_buf, 6)?;

    if args.zstd_threads > 1 {
        encoder.multithread(args.zstd_threads)?;
    }

    let total_steps = TOTAL_ITERATIONS / args.chunk_size;
    let steps = (0..TOTAL_ITERATIONS).step_by(args.chunk_size).par_bridge();

    rayon::scope(|s| {
        let (tx, rx) = bounded::<Vec<u8>>(args.queue_size);

        s.spawn(|_| {
            let mut writer = BufWriter::with_capacity(256 * 1024 * 1024, encoder);
            let mut blocks_written = 0;

            for block in rx {
                writer.write_all(block.as_slice()).unwrap();
                blocks_written += 1;
                if blocks_written % 1000 == 0 {
                    println!("{}%", blocks_written as f64 * 100_f64 / total_steps as f64);
                }
            }
            writer.flush().unwrap();
        });

        steps.into_par_iter().for_each_with(tx.clone(), |c, i| {
            let mut buf = Vec::with_capacity(args.chunk_size * 13);
            (0..args.chunk_size).for_each(|j| {
                let input = i + j;
                let mut acc: i16 = 0;

                // Calculate checksum
                for k in 0..11 {
                    acc += (QN[k] as i16) * (((input / DIV_ARR[k]) % 10) as i16);
                }
                acc %= 11;

                if acc <= 1 {
                    acc = 0;
                } else {
                    acc = 11 - acc;
                }
                let comp_val = input * 10 + acc as usize;
                let formatted = format!("{:012}\n", comp_val);
                buf.extend_from_slice(formatted.as_bytes());
            });
            // Send the block to the writer thread
            if let Err(e) = c.send(buf) {
                eprintln!("Failed to send data to writer thread: {}", e);
            }
        });
        drop(tx);
    });

    Ok(())
}
