use clap::Parser;
use quote::ToTokens;
use syn::{visit::Visit, File, Lit, LitStr, Macro};
use walkdir::WalkDir;

/// Parses the given project and prints out metrics reported
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the project directory
    #[clap(value_parser)]
    project_path: String,
}

struct MacroVisitor {
    // Collects the strings associated with specific macros
    macros_of_interest: Vec<String>,
}

impl MacroVisitor {
    fn new() -> Self {
        MacroVisitor {
            macros_of_interest: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for MacroVisitor {
    fn visit_macro(&mut self, i: &'ast Macro) {
        // Convert the macro to a token stream and then to a string for analysis
        let macro_string = i.tokens.to_string();
        let macro_ident = match i.path.get_ident() {
            Some(s) => s.to_string(),
            None => return,
        };
        let metric_macro = match macro_ident.as_str() {
            // https://docs.rs/metrics/latest/metrics/index.html#macros
            "gauge" | "counter" | "histogram" => macro_ident,
            "register_counter" | "register_gauge" =>
            /* not supported yet */
            {
                return
            }
            _ => return,
        };
        println!("metric macro ident {metric_macro} Tokens: {macro_string} ");

        // Attempt to parse the macro tokens as a syn::Expr
        match i.parse_body() {
            Ok(parsed_macro) => {
                // If the macro's first argument is a string literal, extract it
                if let syn::Expr::Lit(expr_lit) = parsed_macro {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        self.macros_of_interest.push(lit_str.value());
                    } else {
                        println!("Not a string literal: {metric_macro}");
                    }
                } else {
                    println!("Not a literal expression {metric_macro}");
                }
            }
            Err(e) => {
                eprintln!("Error while parsing tokens: {e}. Tokens: {macro_string}");
            }
        };
    }
}

fn main() {
    let args = Args::parse();
    let mut visitor = MacroVisitor::new();

    for entry in WalkDir::new(args.project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let content = std::fs::read_to_string(entry.path()).expect("Error reading file");
        let syntax_tree: File = syn::parse_file(&content).expect("Error parsing file");

        visitor.visit_file(&syntax_tree);
    }

    // After visiting all files, print the collected macro strings
    println!("Found macros of interest:");
    for macro_string in visitor.macros_of_interest {
        println!("{}", macro_string);
    }
}
