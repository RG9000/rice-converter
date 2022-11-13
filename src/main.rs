use std::{fs::{self, File}, collections::HashMap, io::Write};

use serde::Deserialize;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// The input config json file
   #[arg(short, long)]
   input: String,

   /// The output format (alacritty)
   #[arg(short, long)]
   format: String,
}

fn decode_config_file(file_contents : &str) -> InputFile
{
	return serde_json::from_str(file_contents).expect("JSON was not in the expected format");
}

fn read_file(file_path : &str) -> String
{
	return fs::read_to_string(file_path).expect("ERROR READING FILE")
}

#[derive(Debug, Deserialize, Clone)]
struct InputFile {
    background: String,
    foreground: String,
    black: String,
    white: String,
    red: String,
    green: String,
    blue: String,
    yellow: String,
    magenta: String,
    cyan: String
}

#[derive(Hash, Eq, PartialEq)]
enum OutputFormat {
    //insert more lines here to add more translations
    Alacritty
}

struct Translation {
    file_name: String,
    input_file: Vec<u8>,
}

fn get_translations() -> HashMap<OutputFormat, Translation>
{
    let hm = HashMap::from([
        //insert more lines here to add more translations
        (OutputFormat::Alacritty, Translation{file_name: ".alacritty.yml".to_string(), input_file: include_bytes!("assets/.alacritty.yml").to_owned().to_vec()})
    ]);

    return hm;
}

fn do_replacements(output_format: &String, input_file: InputFile) -> String
{
    return output_format
        .replace("$blue", &input_file.blue)
        .replace("$green", &input_file.green)
        .replace("$red", &input_file.red)
        .replace("$black", &input_file.black)
        .replace("$white", &input_file.white)
        .replace("$background", &input_file.background)
        .replace("$foreground", &input_file.foreground)
        .replace("$yellow", &input_file.yellow)
        .replace("$magenta", &input_file.magenta)
        .replace("$cyan", &input_file.background)
}


fn main() {
    let args = Args::parse(); 
    let input_file_name = args.input;
    let output_format_name = args.format;
    let translations = get_translations();

    let input_file = decode_config_file(&read_file(&input_file_name));

    let output_format = match output_format_name.as_str() {
        //insert more lines here to add more translations
        "alacritty" => &translations[&OutputFormat::Alacritty],
        _ => panic!("unrecognized format")
    };

    let chosen_output_format = String::from_utf8(output_format.input_file.to_owned()).unwrap();
    let final_output = do_replacements(&chosen_output_format, input_file);

    let mut file = File::create(&output_format.file_name).expect("COULD NOT CREATE THE NEW FILE");
    file.write(final_output.as_bytes()).expect("COULD NOT WRITE TO NEW FILE");

}
