# bcp

A convenient block copy program.  `bcp` is intended to copy contiguous
chunks of files from point A to point B.  Want to cookie-cutter a
piece out of of a file and plop it down in the middle of another file?
Here you go.

The main use case I want for myself is doing light surgery to disk
images, such as "copy this bootloader block into the image at this
offset".

# Usage

```
$ bcp --help
bcp 0.2.0
Simon Heath <icefox@dreamquest.io>
A convenient program for copying blocks of bytes within files.

USAGE:
    bcp [FLAGS] [OPTIONS] <SRC> <DST>

FLAGS:
    -v, --verbose    Verbose output, with progress bar.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --src-offset <src_offset>    The byte offset in the source file to start reading from. Must not be larger than
                                     the file in question. [default: 0]
    -d, --dst-offset <dst_offset>    The byte offset in the destination file to start writing to. Must not be larger
                                     than the file in question, and the file must exist. [default: 0]
    -c, --count <count>              The number of bytes to copy.  Defaults to "all of them", from the `src-offset` to
                                     the end of the file.  Asking to read past the end of the source file is an error.

ARGS:
    <SRC>    The source file to copy from.
    <DST>    The destination file to copy to.  Will create the file if it does not exist.
```

# Why?

Basically, the goal is to make a nicer version of the traditional Unix
utility `dd`, because `dd` is just... weird.  See [the Jargon file
entry](http://www.catb.org/jargon/html/D/dd.html) for it, particularly
lines like "the user interface for it is clearly a prank" and "it has
no exact replacement".  So, why not just make a replacement?

In particular: I want a Unix-y command line interface, I want to be
able to blit large chunks of files from point A to point B, and I
really don't need EBCDIC translation, byte swapping, "fail if the
output file already exists" being classified as a conversion, a
default block size of 512 bytes because that's how big hard drive
blocks were on a PDP-11, or the ability to have it catch a `SIGUSR1`
and print I/O statistics.  I want it to copy bytes, and that's all.

...Okay, and have a progress bar.  But that's it.  Honest!

# License

MIT/Apache-2


