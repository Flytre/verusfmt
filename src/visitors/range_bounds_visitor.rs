use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use crate::VerusParser;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashSet; 


lazy_static! {
    static ref PARENT_IMPL_NAME: Mutex<Option<String>> = Mutex::new(None);
    static ref IS_VALID_VIEW_TO_SEQ: Mutex<bool> = Mutex::new(false);
    static ref ENUM_NAME: Mutex<Option<String>> = Mutex::new(None); // To store the enum name
    static ref ENUM_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new()); // To store all enum names

}

// Define a new struct for your custom visitor
pub struct RangeBoundsVisitor {
    _target_name: String, // Store target_name within RangeBoundsVisitor
}

impl RangeBoundsVisitor {
    pub fn new(target_name: String) -> Self {
        RangeBoundsVisitor { _target_name: target_name } // Return an instance of RangeBoundsVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("fn", RangeBoundsVisitor::visit_function);
        handlers.insert("impl", RangeBoundsVisitor::visit_impl);
        handlers.insert("enum", RangeBoundsVisitor::visit_enum);

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

    // fn visit_enum(
    //     datum: &mut CoreDatum,
    //     pair: Pair<Rule>,
    //     handlers: &dyn HandlerInterface<CoreDatum>,
    // ) {
    
    //     // Reset enum_name for each visit
    //     *ENUM_NAME.lock().unwrap() = None;
    
    //     for inner_pair in pair.clone().into_inner() {
    //         match inner_pair.as_rule() {
    //             Rule::name => {
    //                 *ENUM_NAME.lock().unwrap() = Some(inner_pair.as_str().to_string()); // Capture the enum name
    //             }
    //             _ => {}
    //         }
    //     }
    //     VerusVisitor::visit_all(datum, pair.into_inner(), handlers);

    // }

    
    fn visit_enum(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::name => {
                    let enum_name = inner_pair.as_str().to_string(); // Capture the enum name
                    ENUM_NAMES.lock().unwrap().insert(enum_name);   // Add it to the set
                }
                _ => {}
            }
        }
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    

    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut param_list = None;
        let mut _fn_qualifier = None;
        let mut _req_expr = None;
        let mut param_map = HashMap::new();
        let mut is_spec_mode = false; // Flag to check if fn_mode is "spec"


        /////
        let current_impl_parent_name = PARENT_IMPL_NAME.lock().unwrap().clone();
    
        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::name)
            .expect("Function must have a name")
            .as_str();
    
        let ret_type = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::ret_type);
    
        if let Some(_current_impl_parent_name) = current_impl_parent_name {
            if name == "view" {
                if let Some(ret_type_pair) = ret_type {
                    let ret_type_str = ret_type_pair.as_str();
                    if ret_type_str.contains("Seq<") {
                        {
                            let mut is_valid_view_to_seq = IS_VALID_VIEW_TO_SEQ.lock().unwrap();
                            *is_valid_view_to_seq = Some(true).is_some();
                        }
                    }
                }
            }
        }
        /////
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::param_list => {
                    param_list = Some(inner_pair.clone());
                }
                Rule::fn_qualifier => {
                    _fn_qualifier = Some(inner_pair.clone());
                    let inner_qualifier = inner_pair.clone().into_inner();
                    for inner_inner_qualifier in inner_qualifier {
                        match inner_inner_qualifier.as_rule() {
                            Rule::requires_clause => {
                                _req_expr = Some(inner_inner_qualifier.clone());
                            }
                            _ => {}
                        }
                    }
                }
                Rule::fn_mode => {
                    if inner_pair.as_str() == "spec" {
                        is_spec_mode = true;
                    }
                }
                _ => {}
            }
        }

        if is_spec_mode {
            // spec fn's dont accept pre/post conditions
            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
            return;
        }

        let mut current_param_name = None;
        if let Some(ref param_list) = param_list {
            for inner_param in param_list.clone().into_inner() {
                for param_vals in inner_param.clone().into_inner() {
                    let rule_str = format!("{:?}", param_vals.as_rule());

                    match rule_str.as_str() {
                        "pat_no_top_alt" => {
                            current_param_name = Some(param_vals.as_str().to_string());
                        }
                        "type" => {
                            if let Some(param_name) = current_param_name.take() {
                                param_map.insert(param_name, param_vals.as_str().to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // for (param, param_type) in &param_map {
        //     println!("Parameter: {}, Type: {}", param, param_type);
        // }

        // if let (Some(ref fn_qualifier), Some(ref req_expr)) = (fn_qualifier, req_expr) {
        //     println!("fn_qualifier = {:?}", fn_qualifier.as_str());
        //     println!("req_expr = {:?}", req_expr.as_str());
        // }

        // Construct the new function representation
        let mut full_string = String::new();
        let new_requires_expression = Self::generate_requires_expression(datum, &param_map, name);

        for (i, inner) in pair.clone().into_inner().enumerate() {
            match inner.as_rule() {
                Rule::fn_qualifier => {
                    let mut fn_qualifier_string = String::new();
                    let mut requires_found = false; // Flag to check if requires_clause exists

                    for qualifier_part in inner.clone().into_inner() {
                        match qualifier_part.as_rule() {
                            Rule::requires_clause => {
                                requires_found = true; // found a requires_clause
                                fn_qualifier_string.push_str(qualifier_part.as_str());

                                // Append the new requires expression if it exists
                                if let Some(ref new_expr) = new_requires_expression {
                                    fn_qualifier_string.push_str("\n");
                                    fn_qualifier_string.push_str(new_expr);
                                    fn_qualifier_string.push_str(",\n");
                                }
                            }
                            _ => {
                                // Append any other parts of fn_qualifier without modification
                                fn_qualifier_string.push_str(qualifier_part.as_str());
                            }
                        }
                    }

                    // If no requires_clause was found, add a new one if new_requires_expression is available
                    if !requires_found && new_requires_expression.is_some() {
                        let mut pre_fn_qualifier_string = String::new();
                        pre_fn_qualifier_string.push_str(" requires ");
                        if let Some(ref new_expr) = new_requires_expression {
                            pre_fn_qualifier_string.push_str(new_expr);
                        }
                        fn_qualifier_string = pre_fn_qualifier_string + &fn_qualifier_string;
                    }
                    // Append the constructed fn_qualifier_string to full_string
                    full_string.push_str(&fn_qualifier_string);
                }
                _ => {
                    // For all non-fn_qualifier cases, add the string representation to full_string
                    full_string.push_str(inner.as_str());
                }
            }
            // Add a newline after each inner string, except the last one
            if i < pair.clone().into_inner().count() - 1 {
                full_string.push_str("\n");
            }
        }
        if let Some(function_pair) = VerusParser::str_to_function(&full_string) {
            datum.program_mut().push_str(function_pair.as_str());
        } else {
            println!("Failed to parse the function.");
            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        }
    }

    //[TODO: Expand to handle more than numeric types]
    fn generate_requires_expression(
        datum: &mut CoreDatum,
        param_map: &HashMap<String, String>,
        function_name: &str, // Add function name as a parameter
    ) -> Option<String> {
        use regex::Regex;
    
        // List of recognized numerical types
        let numerical_types = ["int", "nat", "u32", "i32", "u64", "f32", "f64"];
        let enum_names = ENUM_NAMES.lock().unwrap(); // Access the set of enum names
    
        // Regex to check if function name ends with "_n" where 'n' is a number
        let re = Regex::new(r"_([0-9]+)$").unwrap();
        let bound_override = re.captures(function_name).and_then(|cap| cap.get(1).map(|m| m.as_str()));
    
        // Collect parameters with numerical types
        let mut numerical_params = Vec::new();
        let mut vector_params = Vec::new(); // Store vector type parameters
        let mut recursive_expressions = Vec::new(); // Store recursive expressions
    
        for (param, param_type) in param_map {
            // Check if the param_type matches any of the stored enum names
            if enum_names.contains(param_type.trim_start_matches('&')) {
                // Use the overridden bound if available, otherwise use datum.finite_bound
                let bound = bound_override
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| datum.finite_bound.to_string());
    
                // Add the original recursive expression
                recursive_expressions.push(format!("{}.maxDepth_{}()", param, bound));
    
                // Add the appropriate len() expression based on bound_override
                if bound_override.is_some() {
                    recursive_expressions.push(format!("{}@.len() < {}", param, bound));
                } else {
                    recursive_expressions.push(format!("{}@.len() <= {}", param, bound));
                }
            }
    
            // Check for simple numerical types
            if numerical_types.contains(&param_type.as_str()) {
                numerical_params.push(param.clone());
            }
            // Check for vector types
            else if param_type.starts_with("&Vec<")
                || param_type.starts_with("Vec<")
                || param_type.starts_with("Seq<")
                || param_type.starts_with("Set<")
                || param_type.starts_with("Map<")
            {
                // Add the parameter to vector_params regardless of the inner type
                vector_params.push(param.clone());
            }
        }
    
        // Generate expressions only if there are numerical parameters or vector parameters
        let mut expressions = Vec::new();
    
        // Add expressions for numerical parameters
        if !numerical_params.is_empty() {
            expressions.extend(numerical_params.into_iter().flat_map(|param| {
                vec![
                    format!("{} >= 0", param),
                    format!("{} <= {}", param, datum.finite_bound),
                ]
            }));
        }
    
        // Add expressions for vector parameters (only for length)
        if !vector_params.is_empty() {
            expressions.extend(
                vector_params
                    .into_iter()
                    .map(|param| {
                        if param_map.get(&param).unwrap_or(&String::new()).starts_with("Set<") {
                            // Special handling for Set: Example constraints
                            format!("{}.len() <= {},\n {}.finite()", param, datum.finite_bound, param)
                        } else if param_map.get(&param).unwrap_or(&String::new()).starts_with("Map<") {
                            // Special handling for Map: Example constraints
                            format!(
                                "{}.dom().len() <= {},\n {}.dom().finite()",
                                param, datum.finite_bound, param
                            )
                        } else {
                            // General handling for Vec or Seq
                            format!("{}.len() <= {}", param, datum.finite_bound)
                        }
                    }),
            );
        }
    
        // Add recursive expressions
        expressions.extend(recursive_expressions);
    
        // If we have any expressions, join and return them
        if !expressions.is_empty() {
            Some(expressions.join(",\n "))
        } else {
            None // No parameters found
        }
    }
    
    
}    
