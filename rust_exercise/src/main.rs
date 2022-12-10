use chrono::{DateTime, NaiveTime, NaiveDate, NaiveDateTime, Local, Duration};
use std::env;
use std::fs;
use std::process;
use regex::Regex;
use regex::CaptureMatches;
use lazy_static::lazy_static;
use std::io::Write;


fn main()-> Result<(), std::io::Error> {
    Print("Started.");
    
    /* (1) */
    let mut inputpath = String::from("");
    let mut outputpath = String::from("");

    // Arguments
    let args: Vec<String> = env::args().collect();
    if !DealingWithArgsOk(args, &mut inputpath, &mut outputpath) { process::exit(0x0100);}

    // Open and read all the file
    Print(&format!("Read the input file \"{}\".", inputpath));
    let content = fs::read_to_string(inputpath)
        .expect("Should have been able to read the file");

    /* (2) */
    let strt = chrono::offset::Local::now();    // to measure the time
    Print(format!("Start measuring the time at {}.", datetime_to_HHMMSSffff(strt)).as_str());

    // Extract the datas from the input file
    Print("Extract the datas.");
    let matches = ExtractDatasFromString(&content);

    // Reshape the datas
    Print("Reshape.");
    let mut values = ReshapeDatas(matches);

    // Sort the datas
    Print("Sort.");
    values.sort();
    values.reverse();

    // Measure the time used for the step 2
    let dt = chrono::offset::Local::now()-strt;
    Print(format!("Stop measuring the time. dt: {}.{}", dt.num_seconds(), dt.num_milliseconds()).as_str());

    /* (3, 4) */
    // Write the ouptut file
    Print(format!("Write the output file \"{}\".", outputpath).as_str());
    WriteFile(outputpath, values, dt);

    Print("Finished.");
    Ok(())
}

//#################################################################################################
// Functions
//#############################################################################################
fn datetime_to_HHMMSSffff(time: DateTime<Local>) -> String {
    // let time_as_string = time.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    lazy_static! {
        static ref REG_HHMMSSFFFF: Regex = Regex::new(r"(\d{2}:\d{2}:\d{2}\.\d{4})").unwrap();
    }
    REG_HHMMSSFFFF.captures(time.format("%Y-%m-%d %H:%M:%S.%f").to_string().as_str())
        .unwrap()[0]
        .to_string()
}


fn ExtractDatasFromString(content: &String) -> CaptureMatches {
    lazy_static! {
        static ref REG: Regex = Regex::new("(?:\"PA)(\\d+?)(?::proALPHA:)([^\":\\r\\n]+)(?:\")").unwrap();
    }
    let matches: CaptureMatches = REG.captures_iter(content.as_str());
    matches
}

fn WriteFile(outputpath: String, values: Vec<String>, dt: Duration){
    let time = DateTime::parse_from_rfc3339("2000-01-01T00:00:00+01:00").unwrap() + dt;
    let time: DateTime<Local> = DateTime::<Local>::from(time);
    
    let mut output = std::fs::File::create(outputpath).unwrap();
    
    writeln!(output, "{}", format!("Zeit {}", datetime_to_HHMMSSffff(time))).expect("Error when writing");

    for line in values {
        writeln!(output, "{}", line).expect("Error when writing");
    }
}

fn ReshapeDatas(matches: CaptureMatches) -> Vec<String> {
    let mut values: Vec<String> = Vec::with_capacity(10_000_000);

    for m in matches {
        values.push(format!("{}{}", &m[2], &m[1]));
    }

    values
}

fn DealingWithArgsOk(args: Vec<String>, inputpath: &mut String, outputpath: &mut String) -> bool
{
    let in_path = env::current_dir();
    match in_path {
        Ok(p) => {
            *inputpath = format!("{}/{}", p.display(), "Input.txt");
            *outputpath = format!("{}/{}", p.display(), "Output.txt");
            
        },
        Err(_)    => { Print("Impossible get the actual path.") }
    }

    // No args. Use default parameters
    if args.len() <= 1 {
        return true;
    }
    // Help
    if args[1]=="--help" || args[1]=="-h" {
        DisplayHelp();
        return false;
    }
    // Path of input and output files
    else if args[1] != "" && args.len() > 2 && args[2] != "" {
        *inputpath = format!("{}", args[1]);
        *outputpath = format!("{}", args[2]);
    }
    // Bad arguments
    else {
        DisplayHelp();
        return false;
    }

    return true;
}

fn DisplayHelp(){
    println!("cs_exercise [--help]");
    println!("cs_exercise [full_input_file_path] [full_output_file_path]");
    println!("Exemples: ");
    println!("\tcs_exercise --help");
    println!("\tcs_exercise inputfile1.txt outputfile1.txt");
}

fn Print(txt:&str){
    println!("{:?}: {}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S.%3f").to_string(), txt);
}
