use kseq::record::Fastx;
pub fn write_fastq(rec: Fastx<'_>) {
    println!(
        "{} {}\n{}\n{}\n{}", 
        "@".to_string() + rec.head(), rec.des(), 
        rec.seq(), 
        "+", 
        rec.qual()
    );
}