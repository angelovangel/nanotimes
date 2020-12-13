# nanotimes

### Description

`nanotimes` is a command line program for working with the time stamps of fastq files from 
Oxford Nanopore (ONT) sequencing. It is written in Rust and is thus very fast and has no runtime dependencies or overhead.
By reading the time stamps in the description field of the fastq file, it can be used to:

- get the start and end time of sequencing (min and max time stamp)
- filter reads based on time (minutes)

I have previously [written a solution]() for this using `R` and `seqkit`, but it turned out to be too slow (days!) for big ONT runs.

### Installation and usage

### References