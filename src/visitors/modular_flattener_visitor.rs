use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use crate::VerusParser;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PARENT_FUNCTION_NAME_MAP: Mutex<HashMap<String, String>> =
        Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_PARAM_LIST_MAP: Mutex<HashMap<String, Vec<String>>> =
        Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_QUALIFIER_MAP: Mutex<HashMap<String, String>> =
        Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_NAME: Mutex<String> = Mutex::new(String::new());
}

// Define a new struct for your custom visitor
pub struct ModularFlattenerVisitor {}

impl ModularFlattenerVisitor {
    pub fn new() -> Self {
        ModularFlattenerVisitor {} // Return an instance of ModularFlattenerVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("fn", ModularFlattenerVisitor::visit_function);
        handlers.insert(
            "verus_macro_use",
            ModularFlattenerVisitor::visit_verus_macro_use,
        );
        handlers.insert("expr", ModularFlattenerVisitor::visit_expr);
        handlers.insert("let_stmt", ModularFlattenerVisitor::visit_let_stmt);

        handlers
    }

    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(
            datum,
            pairs,
            &handler_map as &dyn HandlerInterface<CoreDatum>,
        );
    }

    fn visit_let_stmt(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut inner_pairs = pair.clone().into_inner();
        let mut receiving_var: Option<String> = None;
        let mut function_name: Option<String> = None;
        let mut args: Vec<String> = Vec::new(); // To store argument values

        while let Some(inner_pair) = inner_pairs.next() {
            match inner_pair.as_rule() {
                Rule::pat => {
                    let pat_pair = inner_pair.clone();
                    receiving_var = Some(pat_pair.as_str().to_string());
                }
                Rule::expr => {
                    let expr_pair = inner_pair.clone();
                    let mut inner_expr_pairs = expr_pair.clone().into_inner();
                    while let Some(inner_expr_pair) = inner_expr_pairs.next() {
                        match inner_expr_pair.as_rule() {
                            Rule::expr_inner => {
                                let nested_pairs = inner_expr_pair.clone().into_inner();
                                if let Some(function_pair) = nested_pairs
                                    .clone()
                                    .find(|p| p.as_rule() == Rule::path_expr_no_generics)
                                {
                                    function_name = Some(function_pair.as_str().to_string());
                                }
                            }
                            Rule::arg_list => {
                                let arg_str = inner_expr_pair
                                    .into_inner()
                                    .map(|p| p.as_str().to_string())
                                    .collect::<Vec<String>>()
                                    .join(", ");
                                args = arg_str.split(',').map(|s| s.trim().to_string()).collect();
                                // println!("Function arguments: {:?}", args);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        if let (Some(function_name), Some(receiving_var)) =
            (function_name.clone(), receiving_var.clone())
        {
            //   println!("Let Inner = {:?} {:?}", pair.as_str(), pair.as_rule());
            if let Some(function_body) = datum.fn_map.get(&function_name) {
                let mut updated_function_body: Option<String> = None; // store fn body (minus last line)
                let mut assert_clauses: Vec<String> = Vec::new(); // store requires expr as assertions
                let mut assignments: Vec<String> = Vec::new(); // store params for local assignment

                let parent_args = {
                    let parent_args_map = PARENT_FUNCTION_PARAM_LIST_MAP.lock().unwrap();
                    parent_args_map.get(&function_name).cloned()
                };

                if let Some(parent_args) = parent_args {
                    if parent_args.len() == args.len() {
                        // Create a list of assignment strings
                        assignments = parent_args
                            .iter()
                            .zip(args.iter())
                            .filter_map(|(parent_arg, arg)| {
                                // Skip if the argument names are the same
                                if parent_arg == arg {
                                    None
                                } else {
                                    Some(format!("let {} = {};", parent_arg, arg))
                                }
                            })
                            .collect();
                        // println!("Assignments: {:?}", assignments);
                    } else {
                        println!(
                            "Argument lists for function {} do not match in length",
                            function_name
                        );
                        return;
                    }
                }

                // Get qualifiers if available
                let parent_qualifiers = {
                    let parent_quals_map = PARENT_FUNCTION_QUALIFIER_MAP.lock().unwrap();
                    parent_quals_map.get(&function_name).cloned()
                };

                if let Some(parent_qualifiers) = parent_qualifiers {
                    if let Some(function_pair) =
                        VerusParser::str_to_fn_qualifier(&parent_qualifiers)
                    {
                        let mut inner_pairs = function_pair.clone().into_inner();

                        while let Some(inner_pair) = inner_pairs.next() {
                            if inner_pair.as_rule() == Rule::requires_clause {
                                let mut requires_inner_pairs = inner_pair.clone().into_inner();

                                while let Some(requires_inner_pair) = requires_inner_pairs.next() {
                                    // Check if the inner pair matches the comma_delimited_exprs_for_verus_clauses rule
                                    if requires_inner_pair.as_rule()
                                        == Rule::comma_delimited_exprs_for_verus_clauses
                                    {
                                        let mut expr_pairs =
                                            requires_inner_pair.clone().into_inner();

                                        while let Some(expr_pair) = expr_pairs.next() {
                                            // Build the assert statement for each expression
                                            let assert_clause =
                                                format!("assert({});", expr_pair.as_str());
                                            assert_clauses.push(assert_clause);
                                        }
                                    }
                                }
                                // println!("Generated assert clauses: {:?}", assert_clauses);
                            }
                        }
                    }
                }

                if let Some(function_pair) = VerusParser::str_to_function(&function_body) {
                    let mut inner_pairs = function_pair.clone().into_inner();
                    let mut _function_terminator = String::new();
                    if let Some(first_inner_pair) = inner_pairs.next() {
                        let mut first_inner_pairs = first_inner_pair.clone().into_inner();

                        while let Some(inner_pair) = first_inner_pairs.next() {
                            if inner_pair.as_rule() == Rule::fn_terminator {
                                _function_terminator = inner_pair.as_str().to_string();

                                let trimmed_function_body =
                                    inner_pair.as_str().trim_matches(|c| c == '{' || c == '}');
                                // println!("Trimmed function body: {}", trimmed_function_body);

                                // Split the body into lines and modify the last line
                                let mut lines: Vec<String> = trimmed_function_body
                                    .lines()
                                    .map(|line| line.trim().to_string())
                                    .collect();
                                if let Some(last_line) = lines.last_mut() {
                                    if last_line.starts_with("return") || !last_line.is_empty() {
                                        let expr = last_line
                                            .trim_start_matches("return")
                                            .trim_end_matches(';')
                                            .trim();
                                        *last_line = format!("let {} = {}; ", receiving_var, expr);
                                    }
                                }

                                updated_function_body = Some(lines.join("\n"));
                            }
                        }
                    }
                } else {
                    println!("Failed to parse function body");
                }
                for assignment in &assignments {
                    datum
                        .program_mut()
                        .push_str(&format!("{}\n ", assignment.as_str()));
                    // println!("{}", assignment);
                }
                for assert_clause in &assert_clauses {
                    datum
                        .program_mut()
                        .push_str(&format!("{}\n ", assert_clause.as_str()));
                    // println!("{}", assert_clause);
                }
                if let Some(updated_function_body) = updated_function_body {
                    datum
                        .program_mut()
                        .push_str(&format!("{}\n ", updated_function_body.as_str()));
                    // println!("Updated function body:\n{}", updated_function_body);
                }
            } else {
                VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
            }
        } else {
            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        }
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
        let param_list = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::param_list);
        let fn_qualifier = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::fn_qualifier);
        println!("Stored the function body for {}", name);

        datum
            .fn_map
            .insert(name.to_string(), pair.as_str().to_string());
        {
            let mut parent_names = PARENT_FUNCTION_NAME_MAP.lock().unwrap();
            if let Some(mode) = mode {
                parent_names.insert(name.to_string(), mode.as_str().to_string());
            } else {
                parent_names.insert(name.to_string(), "fn".to_string());
            }
        }
        {
            let mut parent_param_list = PARENT_FUNCTION_PARAM_LIST_MAP.lock().unwrap();
            if let Some(param_list) = param_list {
                // println!("Parameters for {}:", name);

                let mut params_vec = Vec::new(); // Initialize a vector to hold parameter strings

                for param in param_list.clone().into_inner() {
                    // println!("Param = {:?} {:?}", param.as_str(), param.as_rule());

                    let inner_param = param.clone().into_inner();
                    for innerp in inner_param {
                        if innerp.as_rule() == Rule::pat_no_top_alt {
                            // println!("Inner Param = {:?} {:?}", innerp.as_str(), innerp.as_rule());
                            params_vec.push(innerp.as_str().to_string());
                        }
                    }
                }

                parent_param_list.insert(name.to_string(), params_vec);
            } else {
                parent_param_list.insert(name.to_string(), Vec::new());
            }
        }
        {
            let mut parent_qualifiers = PARENT_FUNCTION_QUALIFIER_MAP.lock().unwrap();
            if let Some(fn_qualifier) = fn_qualifier {
                parent_qualifiers.insert(name.to_string(), fn_qualifier.as_str().to_string());
            } else {
                parent_qualifiers.insert(name.to_string(), "()".to_string());
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
        // println!("Expr = {:?} {:?}", pair.as_str(), pair.as_rule());
    
        let mut inner_pairs = pair.clone().into_inner();
        let mut function_name: Option<String> = None;
        let mut args: Vec<String> = Vec::new(); // To store argument values
        let mut _param_names: Vec<String> = Vec::new(); // To store parameter names
        let mut non_expr_inner_and_arg_list_pairs = Vec::new(); // Collect pairs that are not expr_inner or arg_list


        let mut contains_only_expr_inner_and_arg_list = true;
    
        while let Some(inner_pair) = inner_pairs.next() {
            // println!("inner = {:?} {:?}", inner_pair.as_rule(), inner_pair.as_str());
            match inner_pair.as_rule() {
                Rule::expr_inner => {
                    let nested_pairs = inner_pair.clone().into_inner();
                    if let Some(function_pair) = nested_pairs
                        .clone()
                        .find(|p| p.as_rule() == Rule::path_expr_no_generics)
                    {
                        function_name = Some(function_pair.as_str().to_string());
                    }
                }
                Rule::arg_list => {
                    let arg_str = inner_pair
                        .into_inner()
                        .map(|p| p.as_str().to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    args = arg_str
                        .split(',')
                        .map(|s| s.trim().to_string()) // Get the argument values
                        .collect();
                }
                _ => {
                    non_expr_inner_and_arg_list_pairs.push(inner_pair);

                }
            }
        }
    
        // if !contains_only_expr_inner_and_arg_list {
        //     // If there are other rules, recurse
        //     VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        //     return;
        // }
        // println!(" ----- non_expr_inner_and_arg_list_pairs = {:?} ", non_expr_inner_and_arg_list_pairs);
    
        if let Some(function_name) = function_name {
            if let Some(function_body) = datum.fn_map.get(&function_name) {
                // println!(" ----- function_name = {:?}  -- {:?}", function_name, args);
    
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
                    // Temporary restriction for "fns" (but still allow proof and spec to be flattened)
                    if called_mode != "fn" && parent_mode != "fn" {
                        let mut function_body = function_body.clone();
    
                        // Extract the parameters from the function signature
                        let function_signature = function_body.split('(').nth(1).unwrap_or("");
                        let param_str = function_signature.split(')').next().unwrap_or("");
                        _param_names = param_str
                            .split(',')
                            .map(|s| s.trim().split(':').next().unwrap().trim().to_string())
                            .collect();
    
                        // println!("\nBefore replacement:\n{:?} {:?}", &_param_names, &args);
    
                        Self::replace_params_with_args(&mut function_body, &_param_names, &args);
                        // println!("\nAfter replacement:\n{}", function_body);
    
                        if let Some(body) = Self::extract_function_body(&function_body) {
                            datum
                                .program_mut()
                                .push_str(&format!("({}) ", body.as_str()));
                            for inner_pair in non_expr_inner_and_arg_list_pairs {
                                    // println!("Recursing on rule: {:?}", inner_pair.as_rule());
                                    VerusVisitor::visit(datum, inner_pair, handlers);
                                }
                            return;
                        } else {
                            println!("Could not extract function body.");
                        }
                    }
                }
            }
        }
    
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }
    

    fn replace_params_with_args(
        function_body: &mut String,
        param_names: &Vec<String>,
        args: &Vec<String>,
    ) {
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
        // println!(
        //     "Functions found (fn_map keys): {:?}",
        //     datum.fn_map.keys().collect::<Vec<&String>>()
        // );
        // println!("Function Calls (fn_calls): {:?}", datum.fn_calls);
        datum.program_mut().push_str("}");
    }
}