// 
use std::{io, io::BufReader,fs};
use flate2::bufread;
use bio::io::{fastq, fastq::FastqRead};
use regex::Regex;


//extern crate clap;
use clap::{Arg, ArgGroup, App};

//extern crate chrono;
use chrono::{DateTime, Duration};

fn main() {
    //println!("Hello, world!");
    let matches = App::new("nanotimes")
        .version("0.2.0")
        .author("Angel Angelov <aangeloo@gmail.com>")
        .about("Work with time stamps of ONT fastq files")

        .arg(Arg::with_name("summary")
            .short("s")
            .long("summary")
            .takes_value(false)
            .help("Print run duraion"))

        .arg(Arg::with_name("filter_start")
            .long("filter_start")
            .takes_value(true)
            .help("Filter reads with a timestamp up to <integer> minutes AFTER START"))

        .arg(Arg::with_name("filter_end")
            .long("filter_end")
            .takes_value(true)
            .help("Filter reads with a timestamp up to <integer> minutes BEFORE END"))

        .arg(Arg::with_name("INPUT")
            .index(1)
            .required(true)
            .help("Path to fastq file"))

        .group(ArgGroup::with_name("group")
            .required(true)
            .args(&["summary", "filter_start", "filter_end"])
        )
        
        .get_matches();
    // app main logic - make a vector with time stamps and work on it to find what is needed
    // then use it to filter out the respective records
    // have to read the records two times

    // parse input file
    let infile = matches.value_of("INPUT").unwrap().to_string();

    // define reader and record
    let mut reader = fastq::Reader::new(get_fastq_reader(&infile));
    let mut record = fastq::Record::new();
    let mut reads1 = 0;
    let mut reads2 = 0;
    
    // define RE
    let re = Regex::new(r"start_time=\S*").unwrap(); // \S is "not white space"
    // vector to collect timestamps
    let mut vec = vec![];
    
    // read once, just to find minimum/maximum time stamps
    reader.read(&mut record).expect("Failed to parse fastq record!");
    while !record.is_empty() {

        let desc = record.desc().unwrap();
        let tstamp = re.find(desc)
            .expect("Could not find start time string!")
            .as_str();
            //.to_owned();

        // parse the extracted string as DateTime to work on it later, put it in the vec
        // [11..] because the string is <start_time=2019-10-30T10:18:24Z>
        // and I need
        // <2019-10-30T10:18:24Z>
        let tstamp_rfc = DateTime::parse_from_rfc3339(&tstamp[11..]) 
            .expect("Failed to parse datetime!");
        
        vec.push(tstamp_rfc);
        reads1 += 1;

        reader.read(&mut record).expect("Failed to parse fastq record!");
    }

    let min_timestamp = vec.iter().min().unwrap();
    let max_timestamp = vec.iter().max().unwrap();
    let duration = max_timestamp.signed_duration_since(*min_timestamp);
    //println!("vec is: {:?}", &vec[1..10]);

    // case summary
    if matches.is_present("summary") {

        println!("Total reads:    {}", reads1);
        println!("Earliest time:  {}", min_timestamp);
        println!("Latest time:    {}", max_timestamp);
        println!("Duration [min]: {}", duration.num_minutes());
        println!("Rate:           {} reads per minute", reads1 / duration.num_minutes());
    
    // case filter start or filter end
    } else if matches.is_present("filter_start") | matches.is_present("filter_end") {
    // parse argument value

    

    

    //second pass to filter reads based on min and max that were found in the first pass
    //
    let mut reader2 = fastq::Reader::new(get_fastq_reader(&infile));
    let mut record2 = fastq::Record::new();

    reader2.read(&mut record2).expect("Failed to parse fastq record!");

    while !record2.is_empty() {
        let mut writer = fastq::Writer::new(io::stdout());
        let desc = record2.desc().unwrap();
        let tstamp = re.find(desc)
            .expect("Could not find start time string!")
            .as_str();
            //.to_owned();

        // parse the extracted string as DateTime to work on it later, put it in the vec
        // [11..] because the string is <start_time=2019-10-30T10:18:24Z>
        // and I need
        // <2019-10-30T10:18:24Z>
        let tstamp_rfc = DateTime::parse_from_rfc3339(&tstamp[11..]) 
            .expect("Failed to parse datetime!");

        // case filter_start
        if matches.is_present("filter_start") {
            let filterminutes_start = matches
                    .value_of("filter_start")
                    .unwrap()
                    .trim()
                    .parse::<i64>()
                    .expect("Failed to parse argument!");

            if tstamp_rfc < *min_timestamp + Duration::minutes(filterminutes_start) {
                writer.write_record(&mut record2).expect("Failed to write fastq record!");
                reads2 += 1;
            }
        // case filter end
        } else if matches.is_present("filter_end") {
            let filterminutes_end = matches
                    .value_of("filter_end")
                    .unwrap()
                    .trim()
                    .parse::<i64>()
                    .expect("Failed to parse argument!");

            if tstamp_rfc > *max_timestamp - Duration::minutes(filterminutes_end) {
                writer.write_record(&mut record2).expect("Failed to write fastq record!");
                reads2 += 1;
            }

        }
        
        reader2.read(&mut record2).expect("Failed to parse fastq record!");

        }
    eprintln!("{}: {} out of {} reads filtered", infile, reads2, reads1);
    } // case filter start or filter end
    
} //main



// fastq reader, file as arg, decide based on extension
fn get_fastq_reader(path: &String) -> Box<dyn (::std::io::Read)> {
    if path.ends_with(".gz") {
        let f = fs::File::open(path).unwrap();
        Box::new(bufread::MultiGzDecoder::new(BufReader::new(f)))
    } else {
        let f = fs::File::open(path).unwrap();
        Box::new(BufReader::new(f))
        //Box::new(fs::File::open(path).unwrap())
    }
}