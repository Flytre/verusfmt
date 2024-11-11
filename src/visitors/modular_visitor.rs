use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitors::visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;
use regex::Regex;
use std::sync::{Mutex};
use lazy_static::lazy_static;


lazy_static! {
    static ref PARENT_FUNCTION_NAME_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_NAME: Mutex<String> = Mutex::new(String::new());

}

// Define a new struct for your custom visitor
pub struct ModularVisitor {
    target_name: String, // Store target_name within ModularVisitor
}

impl ModularVisitor {
    pub fn new(target_name: String) -> Self {
        ModularVisitor { target_name } // Return an instance of ModularVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("fn", ModularVisitor::visit_function);
        handlers.insert("verus_macro_use", ModularVisitor::visit_verus_macro_use);
        handlers.insert("expr", ModularVisitor::visit_expr);

        handlers
    }

    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(datum, pairs, &handler_map as &dyn HandlerInterface<CoreDatum>);
    }


    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::name)
            .expect("Function must have a name")
            .as_str();
        let mode = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::fn_mode);
        println!("Stored the function body for {}", name);

        datum
            .fn_map
            .insert(name.to_string(), pair.as_str().to_string());
        {
            let mut parent_names = PARENT_FUNCTION_NAME_MAP.lock().unwrap();
            if let Some(mode) = mode {
                parent_names.insert(name.to_string(), mode.as_str().to_string());
            }else{
                parent_names.insert(name.to_string(), "fn".to_string());
            }
        }
        {
            let mut parent_name = PARENT_FUNCTION_NAME.lock().unwrap();
            *parent_name = name.to_string();
        }

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut inner_pairs = pair.clone().into_inner();
    
        let mut function_name: Option<String> = None;
        let mut args: Vec<String> = Vec::new(); // To store argument values
        let mut param_names: Vec<String> = Vec::new(); // To store parameter names

        while let Some(inner_pair) = inner_pairs.next() {
            match inner_pair.as_rule() {
                Rule::expr_inner => {
                    let mut nested_pairs = inner_pair.clone().into_inner();
                    if let Some(function_pair) = nested_pairs
                        .clone()
                        .find(|p| p.as_rule() == Rule::path_expr_no_generics)
                    {
                        function_name = Some(function_pair.as_str().to_string());
                    }
                }
                Rule::arg_list => {
                    let arg_str = inner_pair.into_inner().map(|p| p.as_str().to_string()).collect::<Vec<String>>().join(", ");
                    args = arg_str
                        .split(',')
                        .map(|s| s.trim().to_string()) // Get the argument values
                        .collect();
    
                    // Print the extracted arguments for debugging
                    println!("Extracted Arguments: {:?}", args);
                }
                _ => {}
            }
        }
    
        if let (Some(function_name)) = (function_name) {
            println!("Expr B = {:?} {:?}", pair.as_str(), pair.as_rule());
            // println!("Function called: {} with args: {:?}", function_name, args);
            // let parent_mode = {
            //     let parent_names = PARENT_FUNCTION_NAME.lock().unwrap();
            //     parent_names.get(&function_name).cloned()
            // };

            // let called_function_mode = {
            //     let parent_names = PARENT_FUNCTION_NAME.lock().unwrap();
            //     parent_names.get(&function_name).cloned()
            // };
    
            // Retrieve the function body from the fn_map
            if let Some(function_body) = datum.fn_map.get(&function_name) {
                println!("Expr = {:?} {:?}", pair.as_str(), pair.as_rule());
                
                let parent_name = PARENT_FUNCTION_NAME.lock().unwrap().clone();
                let parent_mode = {
                    let parent_map = PARENT_FUNCTION_NAME_MAP.lock().unwrap();
                    parent_map.get(&parent_name).cloned()
                };

                // Retrieve the mode of the called function
                let called_function_mode = {
                    let parent_map = PARENT_FUNCTION_NAME_MAP.lock().unwrap();
                    parent_map.get(&function_name).cloned()
                };
                if let (Some(parent_mode), Some(called_mode)) = (parent_mode, called_function_mode) {

                    // Print out the parent and called function details for debugging
                    println!("Parent Function: {}, Mode: {:?}", parent_name, parent_mode.as_str().to_string());
                    println!("Called Function: {}, Mode: {:?}", function_name, called_mode.as_str().to_string());
                    println!("equal = {:?}", parent_mode.as_str().to_string() == called_mode.as_str().to_string());
                    if(parent_mode.as_str().to_string() == called_mode.as_str().to_string() && called_mode.as_str().to_string() != "fn".to_string()){ // temporary restriction for "fns"
                                   // Replace the function parameters with the actual arguments
                    let mut function_body = function_body.clone();
                    // Now extract the parameters from the function signature
                    let function_signature = function_body.split('(').nth(1).unwrap_or(""); // Extract the part after the '('
                    let param_str = function_signature.split(')').next().unwrap_or(""); // Extract the part before the ')'
                    
                    param_names = param_str
                        .split(',')
                        .map(|s| s.trim().split(':').next().unwrap().trim().to_string()) // Get parameter names before the ":"
                        .collect();

                    // Print the extracted parameter names for debugging
                    // println!("Extracted Parameter Names: {:?}", param_names);
                    Self::replace_params_with_args(&mut function_body, &param_names, &args);
                    // println!("\nAfter replacement:\n{}", function_body);
                    
                    if let Some(body) = Self::extract_function_body(&function_body) {
                        datum.program_mut().push_str(&format!("({}) ", body.as_str()));
                        return;

                    } else {
                        // VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                        println!("Could not extract function body.");
                    }

                    }
                }
            } 
               

        }
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }
    
    fn replace_params_with_args(function_body: &mut String, param_names: &Vec<String>, args: &Vec<String>) {
        for (param, arg) in param_names.iter().zip(args.iter()) {
            // Create a regex pattern to match only the whole word `param`
            let pattern = format!(r"\b{}\b", regex::escape(param));
            let regex = Regex::new(&pattern).expect("Invalid regex pattern");
    
            // Replace occurrences of the whole word with the argument
            *function_body = regex.replace_all(function_body, arg.as_str()).to_string();
        }
    }
    

    fn extract_function_body(function_body: &str) -> Option<String> {
        // Find the first opening brace `{` and the first closing brace `}`
        if let Some(start) = function_body.find('{') {
            if let Some(end) = function_body.rfind('}') {
                if start < end {
                    return Some(function_body[start + 1..end].trim().to_string());
                }
            }
        }
        None // If no braces are found, return None
    }

    

    fn visit_verus_macro_use(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("verus!{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        println!(
            "Functions found (fn_map keys): {:?}",
            datum.fn_map.keys().collect::<Vec<&String>>()
        );
        println!("Function Calls (fn_calls): {:?}", datum.fn_calls);
        datum.program_mut().push_str("}");

    }
}
