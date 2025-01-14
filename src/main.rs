use kseq::parse_path;
use regex::{Regex, RegexSet};


//extern crate clap;
use clap::{Arg, ArgGroup, App};

//extern crate chrono;
use chrono::{DateTime, Duration};

use nanotimes::{self, write_fastq};
fn main() {
    //println!("Hello, world!");
    let arg_matches = App::new("nanotimes")
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
    // use it to filter out the respective records

    let infile = arg_matches.value_of("INPUT").unwrap().to_string();

    let mut records = parse_path(&infile).unwrap();
    let mut reads1 = 0;
    let mut reads2 = 0;
    
    // define RE
    let str1 = r"start_time=\S*";
    let str2 = r"st:Z:\S*";
    let re1 = Regex::new(str1).unwrap(); // \S is "not white space"
    let re2 = Regex::new(str2).unwrap();
    let regexset = RegexSet::new(&[ str1, str2 ]).unwrap();
    // vector to collect timestamps
    let mut vec = vec![];

    while let Some(record) = records.iter_record().unwrap() {
        let desc = record.des();
        //println!("{}", desc);
        let re_matches = regexset.matches(desc);

        let tstamp = if re_matches.matched(0) {
            re1.find(desc).expect("Could not find start time string!").as_str()
        } else if re_matches.matched(1) {
            re2.find(desc).expect("Could not find start time string!").as_str()
        } else {
            "No match"
        };

        let tstamp_index = if re_matches.matched(0) {
            11
        } else if re_matches.matched(1) {
            5
        } else {
            0
        };

        // parse the extracted string as DateTime to work on it later, put it in the vec
        // [11..] because the string is <start_time=2019-10-30T10:18:24Z>
        // and I need
        // <2019-10-30T10:18:24Z>
        let tstamp_rfc = DateTime::parse_from_rfc3339(&tstamp[tstamp_index..]) 
            .expect("Failed to parse datetime!");
        
        vec.push(tstamp_rfc);
        reads1 += 1;
    }

    // case summary
    let min_timestamp = vec.iter().min().unwrap();
    let max_timestamp = vec.iter().max().unwrap();
    let duration = max_timestamp.signed_duration_since(*min_timestamp);
    
    if arg_matches.is_present("summary") {
        eprintln!("Total reads:    {}", reads1);
        eprintln!("Earliest time:  {}", min_timestamp);
        eprintln!("Latest time:    {}", max_timestamp);
        eprintln!("Duration [min]: {}", duration.num_minutes());
        eprintln!("Rate:           {} reads per minute", reads1 / duration.num_minutes());
    } else if arg_matches.is_present("filter_start") | arg_matches.is_present("filter_end") {
        // second pass
        let mut records2 = parse_path(&infile).unwrap();
        while let Some(record2) = records2.iter_record().unwrap() {
            let desc = record2.des();
            let re_matches = regexset.matches(desc);

            let tstamp = if re_matches.matched(0) {
                re1.find(desc).expect("Could not find start time string!").as_str()
            } else if re_matches.matched(1) {
                re2.find(desc).expect("Could not find start time string!").as_str()
            } else {
                "No match"
            };

            let tstamp_index = if re_matches.matched(0) {
                11
            } else if re_matches.matched(1) {
                5
            } else {
                0
            };
            
            let tstamp_rfc = DateTime::parse_from_rfc3339(&tstamp[tstamp_index..]) 
            .expect("Failed to parse datetime!");
            
            // case filter_start
            if arg_matches.is_present("filter_start") {
                let filterminutes_start = arg_matches
                    .value_of("filter_start")
                    .unwrap()
                    .trim()
                    .parse::<i64>()
                    .expect("Failed to parse argument!");

                if tstamp_rfc < *min_timestamp + Duration::minutes(filterminutes_start) {
                    write_fastq(record2);
                    reads2 += 1;
                }
            
            } else if arg_matches.is_present("filter_end") {
                let filterminutes_end = arg_matches
                    .value_of("filter_end")
                    .unwrap()
                    .trim()
                    .parse::<i64>()
                    .expect("Failed to parse argument!");

                if tstamp_rfc > *max_timestamp - Duration::minutes(filterminutes_end) {
                    write_fastq(record2);
                    reads2 += 1;
                }
                
            }
    }
    eprintln!("{} out of {} reads filtered ({:.2} %)", reads2, reads1, reads2 as f64 /reads1 as f64 *100.0);
    } 
}//main
