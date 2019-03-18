use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Seek};
use std::path::PathBuf;
use std::process;
use structopt::{StructOpt, clap::AppSettings};

/// Size of the copy buffer to use: 1 MB.
const BUFSIZE: usize = 1024 * 1024;

#[derive(Debug, StructOpt)]
#[structopt(raw(global_settings="&[AppSettings::DeriveDisplayOrder]"))]
struct Opt {
    /// The source file to copy from.
    #[structopt(short = "s", long = "src", parse(from_os_str))]
    src: PathBuf,

    /// The destination file to copy to.  Will create the file
    /// if it does not exist.
    #[structopt(short = "d", long = "dst", parse(from_os_str))]
    dst: PathBuf,

    /// The byte offset in the source file to start reading from.
    /// Must not be larger than the file in question.
    #[structopt(short = "i", long = "src-offset", default_value = "0")]
    src_offset: u64,

    /// The byte offset in the destination file to start writing to.
    /// Must not be larger than the file in question, and the file
    /// must exist.
    #[structopt(short = "o", long = "dst-offset", default_value = "0")]
    dst_offset: u64,

    /// The number of bytes to copy.  Defaults to "all of them",
    /// from the `src-offset` to the end of the file.  Asking to
    /// read past the end of the source file is an error.
    #[structopt(short = "c", long = "count")]
    count: Option<u64>,
}

/// Print an error message and quit.
fn error(msg: &str) -> ! {
    eprintln!("ERROR: {}", msg);
    process::exit(1)
}

/// Exits if the command line options don't make sense.
fn sanity_check(opt: &Opt) {
    // Check src file length.
    let src_metadata = opt.src.metadata().unwrap_or_else(|e| {
        let errmsg = format!("Could not get metadata for source file: {:?}", e);
        error(&errmsg)
    });
    let src_len = src_metadata.len();
    // TODO: Double-check for off-by-one
    if src_len <= opt.src_offset {
        error("source offset > source file size");
    }

    // Check count is valid
    if let Some(c) = opt.count {
        // TODO: Double-check for off-by-one
        if c + opt.src_offset > src_len {
            error("Count + source offset > source file size");
        }
    }

    // Check dest file length and properties.
    if opt.dst.exists() {
        if opt.dst.is_dir() {
            error("Destination must be a file.");
        }
        let dst_metadata = opt.dst.metadata().unwrap_or_else(|e| {
            let errmsg = format!("Could not get metadata for destination file: {:?}", e);
            error(&errmsg)
        });
        // TODO: Double-check for off-by-one
        if dst_metadata.len() < opt.dst_offset {
            error("destination offset > destination file size");
        }
    }
}

/// Actually do the copy.
fn copy_stuff(opt: &Opt) {
    let mut src = fs::File::open(&opt.src).expect("Should never happen?");
    let mut dst = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&opt.dst)
        .unwrap_or_else(|e| {
            let errmsg = format!("Could not open destination file for writing: {:?}", e);
            error(&errmsg)
        });

    src.seek(io::SeekFrom::Start(opt.src_offset))
        .expect("Should never happen?");
    dst.seek(io::SeekFrom::Start(opt.dst_offset))
        .expect("Should never happen?");

    // ...hmmm.
    // Rewriting our own `io::copy()` and doing our own
    // buffering might honestly be nicer.
    let src = &mut BufReader::with_capacity(BUFSIZE, src);
    let dst = &mut BufWriter::with_capacity(BUFSIZE, dst);

    if let Some(c) = opt.count {
        let src = &mut src.take(c);
        let _ = io::copy(src, dst).unwrap_or_else(|e| {
            let errmsg = format!("Error while copying: {:?}", e);
            error(&errmsg);
        });
    } else {
        let _ = io::copy(src, dst).unwrap_or_else(|e| {
            let errmsg = format!("Error while copying: {:?}", e);
            error(&errmsg);
        });
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    sanity_check(&opt);
    copy_stuff(&opt);
}

#[cfg(test)]
mod test {}
