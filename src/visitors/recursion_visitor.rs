use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use crate::VerusParser;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PARENT_FUNCTION_NAME: Mutex<String> = Mutex::new(String::new());
    static ref RECURSIVE_FNCS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub struct RecursionVisitor {
    target_name: String, // Store target_name within RecursionVisitor
}

impl RecursionVisitor {
    pub fn new(target_name: String) -> Self {
        RecursionVisitor { target_name } // Return an instance of RecursionVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("expr", RecursionVisitor::visit_expr);
        handlers.insert("fn", RecursionVisitor::visit_function);
        handlers.insert("verus_macro_use", RecursionVisitor::visit_verus_macro_use);

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

    fn visit_verus_macro_use(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("verus!{\n");

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);

        let recursive_fncs = RECURSIVE_FNCS.lock().unwrap();

        for (key, value) in recursive_fncs.iter() {
            // println!("Key: {}, Value: {}", key, value);
            datum.program_mut().push_str(value);
        }

        datum.program_mut().push_str("}");
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
        println!("Stored the function body for {}", name);
        datum
            .fn_map
            .insert(name.to_string(), pair.as_str().to_string());
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
        let current_parent_name = PARENT_FUNCTION_NAME.lock().unwrap().clone();
        // println!("Expr = {:?} {:?}:: Parent function name is: {}", pair.as_str(),pair.as_rule(), current_parent_name);

        let mut function_name: Option<String> = None;
        let mut arguments: Option<String> = None;
        let expr_clone = pair.clone();

        for inner_pair in expr_clone.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::expr_inner => {
                    let nested_pairs = inner_pair.clone().into_inner();
                    if let Some(function_pair) = nested_pairs
                        .clone()
                        .find(|p| p.as_rule() == Rule::path_expr_no_generics)
                    {
                        function_name = Some(function_pair.as_str().to_string());
                        if let Some(function_name) = function_name.clone() {}
                    }
                }
                Rule::arg_list => {
                    let args: String = inner_pair
                        .into_inner()
                        .map(|p| p.as_str().to_string())
                        .collect();
                    arguments = Some(args.clone());
                }
                _ => {}
            }
        }

        if let (Some(function_name), Some(arguments)) = (function_name, arguments) {
            if function_name == current_parent_name {
                let finite_bound = datum.finite_bound;

                datum.program_mut().push_str(&format!(
                    "{}_{}({})",
                    function_name, finite_bound, arguments
                ));

                println!(
                    "Recursive Function called!: {} with args: {:?}",
                    function_name, arguments
                );

                // Retrieve the original function body from `fn_map`
                if let Some(original_body) = datum.fn_map.get(&function_name).cloned() {
                    // Generate finite_bound copies of the recursive function
                    for i in 0..=finite_bound {
                        let new_function_name = format!("{}_{}", function_name, i);
                        let mut new_body = original_body.clone();

                        let mut start = 0;
                        let mut first_occurrence = true;

                        while let Some(pos) = new_body[start..].find(&function_name) {
                            let pos = start + pos;

                            // Check if it's the first occurrence
                            if first_occurrence {
                                new_body.replace_range(
                                    pos..pos + function_name.len(),
                                    &new_function_name,
                                );
                                first_occurrence = false;
                            } else {
                                // Replace subsequent occurrences with new_function_name_recursive
                                let new_function_name_recursive = if i == 0 {
                                    format!("{}_0", function_name) // Replace with fnName_0 if i == 0
                                } else {
                                    format!("{}_{}", function_name, i - 1) // Otherwise, use fnName_{i-1}
                                };
                                new_body.replace_range(
                                    pos..pos + function_name.len(),
                                    &new_function_name_recursive,
                                );
                            }
                            start = pos + function_name.len();
                        }

                        // Insert the modified function into the `recursive_fncs` with a cloned body
                        {
                            let mut recursive_fncs = RECURSIVE_FNCS.lock().unwrap(); // lock the Mutex to access the HashMap
                            recursive_fncs.insert(new_function_name.clone(), new_body.clone());
                        }

                        if let Some(function_pair) = VerusParser::str_to_function(&new_body) {
                            // println!("Parsed the function: {:?}", function_pair.as_str());
                        } else {
                            println!("Failed to parse the recursive function.");
                        }
                    }
                }
            } else {
                datum.program_mut().push_str(pair.as_str());
            }
        } else {
            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        }
    }
}
