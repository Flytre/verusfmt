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
    static ref PARENT_IMPL_NAME: Mutex<Option<String>> = Mutex::new(None);
    static ref RECURSIVE_IMPL_FNCS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub struct RecursionVisitor {
    _target_name: String, // Store target_name within RecursionVisitor
}

impl RecursionVisitor {
    pub fn new(target_name: String) -> Self {
        RecursionVisitor { _target_name: target_name } // Return an instance of RecursionVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("expr", RecursionVisitor::visit_expr);
        handlers.insert("fn", RecursionVisitor::visit_function);
        handlers.insert("verus_macro_use", RecursionVisitor::visit_verus_macro_use);
        handlers.insert("impl", RecursionVisitor::visit_impl);
        handlers.insert("assoc_item_list", RecursionVisitor::visit_assoc_item_list);

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


    fn visit_impl(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {

        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::r#type)
            .expect("Function must have a name")
            .as_str();
        {
            let mut parent_name = PARENT_IMPL_NAME.lock().unwrap();
            *parent_name = Some(name.to_string());
        }
       
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        {
            let mut parent_name = PARENT_IMPL_NAME.lock().unwrap();
            *parent_name = None;
        }
       

    }


    fn visit_assoc_item_list(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        let recursive_impl_fncs = RECURSIVE_IMPL_FNCS.lock().unwrap();
        for (_key, value) in recursive_impl_fncs.iter() {
            // println!("Key: {}, Value: {}", key, value);
            datum.program_mut().push_str(value);
        }
        datum.program_mut().push_str("}\n");
    }


    fn visit_verus_macro_use(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("verus!{\n");

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);

        let recursive_fncs = RECURSIVE_FNCS.lock().unwrap();

        for (_key, value) in recursive_fncs.iter() {
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
        // println!(
        //     "Expr = {:?} {:?}:: Parent function name is: {}",
        //     pair.as_str(),
        //     pair.as_rule(),
        //     current_parent_name
        // );
    
        let current_impl_parent_name = PARENT_IMPL_NAME.lock().unwrap().clone();
        let mut impl_self_fn_name: Option<String> = None;
        let mut receiver_seen = false;
    
        let mut function_name: Option<String> = None;
        let mut arguments: Option<String> = None;
        let expr_clone = pair.clone();
    
        for inner_pair in expr_clone.clone().into_inner() {
            // println!("inner = {:?} -- {:?}", inner_pair.as_rule(), inner_pair.as_str());
            match inner_pair.as_rule() {
                Rule::expr_inner => {
                    // Existing handling for simple function names
                    let nested_pairs = inner_pair.clone().into_inner();
                    if let Some(function_pair) = nested_pairs
                        .clone()
                        .find(|p| p.as_rule() == Rule::path_expr_no_generics)
                    {
                        function_name = Some(function_pair.as_str().to_string());
                    }
                }
                Rule::dot_str => {
                    // Dot detected: set the flag to look for the method name next
                    receiver_seen = true;
                }
                Rule::name_ref => {
                    // Capture method name if it follows dot_str
                    if receiver_seen {
                        impl_self_fn_name = Some(inner_pair.as_str().to_string());
                        // receiver_seen = false;
                    }
                }
                Rule::arg_list => {
                    // Collect arguments
                    let args: String = inner_pair
                        .into_inner()
                        .map(|p| p.as_str().to_string())
                        .collect();
                    arguments = Some(args);
                }
                _ => {}
            }
        }
    
        // Decide the final function name
        let final_function_name = impl_self_fn_name.or(function_name.clone());
    
        if let (Some(ref function_name), Some(arguments), Some(ref caller_name)) = (final_function_name, arguments,function_name) {
            if function_name == &current_parent_name {
                let finite_bound = datum.finite_bound;
    
                // Modify the program with the updated function call
                if receiver_seen {
                    datum.program_mut().push_str(&format!(
                        "{}.{}_{}({})",
                        caller_name, function_name, finite_bound, arguments
                    ));
                }else{
                    datum.program_mut().push_str(&format!(
                        "{}_{}({})",
                        function_name, finite_bound, arguments
                    ));
                }
    
                println!(
                    "Recursive Function called!: {} with args: {:?} {:?} {:?}",
                    function_name, arguments,receiver_seen, caller_name
                );
    
                // Retrieve the original function body from `fn_map`
                if let Some(original_body) = datum.fn_map.get(function_name).cloned() {
                    // Generate finite_bound copies of the recursive function
                    for i in 0..=finite_bound {
                        let new_function_name = format!("{}_{}", function_name, i);
                        let mut new_body = original_body.clone();
    
                        let mut start = 0;
                        let mut first_occurrence = true;
    
                        while let Some(pos) = new_body[start..].find(function_name) {
                            let pos = start + pos;
    
                            // Replace occurrences based on whether it's the first or subsequent
                            if first_occurrence {
                                new_body.replace_range(
                                    pos..pos + function_name.len(),
                                    &new_function_name,
                                );
                                first_occurrence = false;
                            } else {
                                let new_function_name_recursive = if i == 0 {
                                    format!("{}_0", function_name)
                                } else {
                                    format!("{}_{}", function_name, i - 1)
                                };
                                new_body.replace_range(
                                    pos..pos + function_name.len(),
                                    &new_function_name_recursive,
                                );
                            }
                            start = pos + function_name.len();
                        }
    
                        // Handle RECURSIVE_FNCS or RECURSIVE_IMPL_FNCS based on the current impl
                        if current_impl_parent_name.is_some() {
                            let mut recursive_impl_fncs = RECURSIVE_IMPL_FNCS.lock().unwrap();
                            recursive_impl_fncs.insert(new_function_name.clone(), new_body.clone());
                        } else {
                            let mut recursive_fncs = RECURSIVE_FNCS.lock().unwrap();
                            recursive_fncs.insert(new_function_name.clone(), new_body.clone());
                        }
    
                        if let Some(_function_pair) = VerusParser::str_to_function(&new_body) {
                            // Successfully parsed
                        } else {
                            println!("Failed to parse the recursive function.");
                        }
                    }
                }
            } else {
                datum.program_mut().push_str(pair.as_str());
            }
        } else {
            // Continue visiting inner pairs if no valid function call found
            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        }
    }
    
}
