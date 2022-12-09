use chrono;
use std::env;
use std::fs;
use std::process;
use std::time;
use regex::Regex;
use regex::CaptureMatches;
use lazy_static::lazy_static;
use std::io::Write;


fn main()-> Result<(), std::io::Error> {
    Print("Started.");
    
    /* (1) */
    let mut inputpath = String::from("");
    let mut outputpath = String::from("");

    // dbg!(args);
    // let args:Vec<String> = env::args().collect();
    let mut args:Vec<String> = Vec::new();
    let str1 = String::from("Input.txt");
    let str2 = String::from("Output.txt");
    args.push(str1);
    args.push(str2);
    if !DealingWithArgsOk(args, &mut inputpath, &mut outputpath) { process::exit(0x0100);}

    // Open and read all the file
    Print(&format!("Read the input file \"{}\".", inputpath));
    let content = fs::read_to_string(inputpath)
        .expect("Should have been able to read the file");
    // Print(&content);

    /* (2) */
    let strt = time::Instant::now();    // to measure the time
    Print("Start measuring the time");

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
    let dt = strt.elapsed();
    Print(format!("Stop measuring the time. dt: {}.", dt.as_secs_f64()).as_str());

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
fn ExtractDatasFromString(content: &String) -> CaptureMatches {
    lazy_static! {
        static ref reg: Regex = Regex::new("(?:\"PA)(?<postnb>\\d+?)(?::proALPHA:)(?<nb>[^\":\\r\\n]+)(?:\")").unwrap();
    }
    let matches: CaptureMatches = reg.captures_iter(content.as_str());
    matches
}

fn WriteFile(outputpath: String, values: Vec<String>, dt: std::time::Duration){
    let mut output = std::fs::File::create(outputpath).unwrap();
    
    write!(output, "{}", format!("Zeit {}", dt.as_secs_f64())).expect("Error when writing");

    for line in values {
        write!(output, "{}", line).expect("Error when writing");
    }
}

fn ReshapeDatas(matches: CaptureMatches) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    // List<string> values = new List<string>(matches.Count);

    for m in matches {
        values.push(format!("{}{}", &m[2], &m[1]));
        print!("{}{}", &m[2], &m[1]);
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
    if args.len() == 0 {
        return true;
    }
    // Help
    if args[0]=="--help" || args[0]=="-h" {
        DisplayHelp();
        return false;
    }
    // Path of input and output files
    else if args[0] != "" && args.len() > 1 && args[1] != "" {
        *inputpath = format!("{}", args[0]);
        *outputpath = format!("{}", args[1]);
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
    println!("{:?}: {}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S:%3f").to_string(), txt);
}
