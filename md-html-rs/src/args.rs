// Parser for command line arguments

use md_html::html::generator::StyleType;

#[derive(Clone)]
pub struct MdHtmlArgs {
    pub input: Option::<String>,
    pub output: Option::<String>,
    pub styles: StyleType,
    pub verbose: bool,
}

impl MdHtmlArgs {
    pub fn new(args: &Vec<String>) -> Result<MdHtmlArgs, String> {
        if args.len() < 3 {
            return Err("not enough arguments".to_string());
        }

        let mut input = Option::<String>::None;
        let mut output = Option::<String>::None;

        let mut styles = StyleType::Default;
        let mut verbose = false;

        let mut arg_index: usize = 1;
        while arg_index < args.len() {
            match args[arg_index].clone().as_str() {
                "-i" => {
                    arg_index += 1;
                    input = Some(args[arg_index].clone());
                },
                "-o" => {
                    arg_index += 1;
                    output = Some(args[arg_index].clone());
                },
                "-v" => {
                    verbose = true;
                },
                "--css-href" => {
                    arg_index += 1;
                    styles = StyleType::External(args[arg_index].clone());
                },
                "--css-file" => {
                    arg_index += 1;
                    styles = StyleType::Inline(args[arg_index].clone());
                },
                arg => { 
                    return Err(format!("unknown argument: {}", arg));
                },
            }
            arg_index += 1;
        }

        if input.is_none() {
            return Err("no input file specified".to_string());
        }

        Ok(MdHtmlArgs { 
            input, 
            output,
            styles, 
            verbose,
         })
    }
}