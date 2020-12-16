# nanotimes

### Description

`nanotimes` is a command line program for working with the time stamps of fastq files from 
Oxford Nanopore (ONT) sequencing. It is written in Rust and is thus very fast and has no runtime dependencies or overhead. Both `fastq` and `fastq.gz` files can be used.
By reading the time stamps in the description field of the fastq file, it can be used to:

- get the start and end time of sequencing (min and max time stamp)
- filter reads based on time (minutes)

I have previously [written a solution](https://github.com/angelovangel/etc/blob/master/bin/filter-times-ont-faster.R) for this using `R` and `seqkit`, but it turned out to be too slow (days!) for big ONT runs.

### Installation and usage

Precompiled binaries are available for MacOS, Linux and Windows, but may not be the latest version. It is best to build it on your system, which is easy:

```bash
# If you don't have the Rust toolchain:
curl https://sh.rustup.rs -sSf | sh

# get the repo
git clone https://github.com/angelovangel/nanotimes.git

# build
cd nanotimes
cargo build --release
```

Try `./target/release/nanotimes --help`. Some usage examples:

```bash
# get an impression about the time stamps
./target/release/nanotimes --summary file.fastq

# filter all reads that are 10 minutes AFTER START (output is stdout)
./target/release/nanotimes --filter_start 10 file.fastq > file-10-min.fastq

# filter all reads that are 10 minutes BEFORE END (output is stdout)
./target/release/nanotimes --filter_end 10 file.fastq > file-10-min.fastq

# for several time points
timepoints=(5 10 20 60 120)
file=path/to/file.fastq

for t in $timepoints; do
    ./target/release/nanotimes --filter_start $t $file > $(basename $file .fastq)-$t-min.fastq;
done

```

### Warning

The parsing of the timestamp relies on a string like `start_time=2019-10-30T10:18:24Z`, which is present in the description field of the ONT fastq files. This may eventually change in the future (depends on Oxford Nanopore), which will break the script.
