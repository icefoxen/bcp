# bcp

A convenient block copy program.  `bcp` is intended to copy large
chunks of files from point A to point B.  Want to cookie-cutter a
piece out of of a file and plop it down in the middle of another file?
Here you go.

Maybe we'll add a nice progress bar with `pbr`.  That sounds nice.

# Why?

Basically, the goal is to make a nicer version of the traditional Unix
utility `dd`, because `dd` is just... weird.  See [the Jargon file
entry](http://www.catb.org/jargon/html/D/dd.html) for it, particularly
lines like "the user interface for it is clearly a prank" and "it has
no exact replacement".  So, why not just make a replacement?

In particular: I want a Unix-y command line interface, I want to be
able to blit large chunks of files from point A to point B, and I
really don't need EBCDIC conversion, byte swapping, "fail if the
output file already exists" being classified as a conversion, a
default block size of 512 bytes because that's how big hard drive
blocks were on a PDP-11 or whatever, or the ability to have it catch a
`SIGUSR1` and print I/O statistics.  I want it to copy bytes, and
that's all.
