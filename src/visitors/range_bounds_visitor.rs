use crate::Rule;
use pest::iterators::{Pair, Pairs}; 
use crate::{visitors::visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;
use crate::VerusParser;

// #[derive(Clone, Debug)]
// pub struct RangeBoundsDatum {
//     pub program: String,
//     pub param_map: HashMap<String,String>,
// }

// impl HasProgram for RangeBoundsDatum {
//     fn program(&self) -> &String {
//         &self.program
//     }

//     fn program_mut(&mut self) -> &mut String {
//         &mut self.program
//     }
// }


// Define a new struct for your custom visitor
pub struct RangeBoundsVisitor {
    target_name: String, // Store target_name within RangeBoundsVisitor
}

impl RangeBoundsVisitor {
    pub fn new(target_name: String) -> Self {
        RangeBoundsVisitor { target_name } // Return an instance of RangeBoundsVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("fn", RangeBoundsVisitor::visit_function);

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
        let mut param_list = None;
        let mut fn_qualifier = None;
        let mut req_expr = None;
        let mut param_map = HashMap::new(); 
        let mut is_spec_mode = false; // Flag to check if fn_mode is "spec"
    
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::param_list => {
                    param_list = Some(inner_pair.clone());
                },
                Rule::fn_qualifier => {
                    fn_qualifier = Some(inner_pair.clone());
                    let inner_qualifier = inner_pair.clone().into_inner();
                    for inner_inner_qualifier in inner_qualifier {
                        match inner_inner_qualifier.as_rule() {
                            Rule::requires_clause => {
                                req_expr = Some(inner_inner_qualifier.clone());
                            },
                            _ => {}
                        }
                    }
                },
                Rule::fn_mode => {
                    if inner_pair.as_str() == "spec" {
                        is_spec_mode = true;
                    }
                },
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
        let new_requires_expression = Self::generate_requires_expression(datum, &param_map); 
    
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
                        fn_qualifier_string.push_str(" requires ");
                        if let Some(ref new_expr) = new_requires_expression {
                            fn_qualifier_string.push_str(new_expr);
                        }
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
 fn generate_requires_expression(datum: &mut CoreDatum, param_map: &HashMap<String, String>) -> Option<String> {
    // List of recognized numerical types
    let numerical_types = ["int", "nat", "u32", "i32", "u64", "f32", "f64"];
    
    // Collect parameters with numerical types
    let mut numerical_params = Vec::new();
    let mut vector_params = Vec::new(); // Store vector type parameters

    for (param, param_type) in param_map {
        // Check for simple numerical types
        if numerical_types.contains(&param_type.as_str()) {
            numerical_params.push(param.clone());
        } 
        
        // Check for vector types
        else if (param_type.starts_with("&Vec<") || param_type.starts_with("Vec<") || param_type.starts_with("Seq<")) {
            // Add the parameter to vector_params regardless of the inner type
            vector_params.push(param.clone());
        }
    }

    // Generate expressions only if there are numerical parameters or vector parameters
    let mut expressions = Vec::new();

    // Add expressions for numerical parameters
    if !numerical_params.is_empty() {
        expressions.extend(numerical_params
            .into_iter()
            .flat_map(|param| {
                vec![
                    format!("{} >= 0", param),
                    format!("{} <= {}", param, datum.finite_bound)
                ]
            }));
    }

    // Add expressions for vector parameters (only for length)
    if !vector_params.is_empty() {
        expressions.extend(vector_params
            .into_iter()
            .map(|param| format!("{}.len() <= {}", param, datum.finite_bound)));
    }

    // If we have any expressions, join and return them
    if !expressions.is_empty() {
        Some(expressions.join(",\n "))
    } else {
        None // No parameters found
    }
}


    


}
