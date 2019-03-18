use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Seek, Write};
use std::path::PathBuf;
use std::process;

use pbr;
use structopt::{clap::AppSettings, StructOpt};

/// Size of the copy buffer to use: 1 MB.
const BUFSIZE: usize = 1024 * 1024;

#[derive(Debug, StructOpt)]
#[structopt(raw(global_settings = "&[AppSettings::DeriveDisplayOrder]"))]
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

    /// Verbose output, with progress bar.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

/// Print an error message and quit.
fn error(msg: &str) -> ! {
    eprintln!("ERROR: {}", msg);
    process::exit(1)
}

/// Exits if the command line options don't make sense.
/// Returns the source file length, 'cause it's handy and
/// there's no point in asking for it twice.
fn sanity_check(opt: &Opt) -> u64 {
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
        //println!("{} < {} ?", dst_metadata.len(), opt.dst_offset);
        if dst_metadata.len() < opt.dst_offset {
            error("destination offset > destination file size");
        }
    } else {
        if opt.dst_offset > 0 {
            error("destination file cannot have an offset if the file does not exist; the results of trying to seek past the end of a file are system-defined and thus probably not what you want.")
        }
    }

    src_len
}

/// Actually do the copy.
fn copy_stuff(opt: &Opt, src_len: u64) {
    let mut src = fs::File::open(&opt.src).expect("Should never happen?");
    let mut dst = fs::OpenOptions::new()
        .write(true)
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

    /*
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
     */

    // TODO: Verify.  The box is annoying.
    // But not having it is also annoying.
    let copy_len = opt.count.unwrap_or(src_len);
    let mut src = src.take(copy_len);

    // Basically stolen from io::copy().
    // We want a little more control over what's happening.
    let mut pb = if opt.verbose {
        let mut bar = pbr::ProgressBar::new(copy_len);
        bar.set_units(pbr::Units::Bytes);
        Some(bar)
    } else {
        None
    };
    let mut buf = vec![0; BUFSIZE];
    let mut written = 0;
    loop {
        let len = match src.read(&mut buf) {
            Ok(0) => break,
            Ok(len) => len,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => {
                let errmsg = format!("Error reading file: {:?}", e);
                error(&errmsg)
            }
        };
        dst.write_all(&buf[..len]).unwrap_or_else(|e| {
            let errmsg = format!("Error reading file: {:?}", e);
            error(&errmsg)
        });
        written += len as u64;
        if let Some(ref mut p) = pb {
            p.add(len as u64);
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let src_len = sanity_check(&opt);
    copy_stuff(&opt, src_len);
}

#[cfg(test)]
mod test {}
