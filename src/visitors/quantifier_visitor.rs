use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitors::visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;
use pest::prec_climber::{PrecClimber, Assoc, Operator};
use crate::VerusParser;
use regex::Regex;

// Define a new struct for your custom visitor
pub struct QuantifierVisitor {
    target_name: String, // Store target_name within QuantifierVisitor
}

impl QuantifierVisitor {
    pub fn new(target_name: String) -> Self {
        QuantifierVisitor { target_name } // Return an instance of QuantifierVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("quantifier_expr", QuantifierVisitor::visit_quantifier); // contains logic for forall and exists
        handlers.insert("quantifier_expr_no_struct", QuantifierVisitor::visit_quantifier); // treat the same as quantifier_expr
        handlers
    }


    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(datum, pairs, &handler_map as &dyn HandlerInterface<CoreDatum>);
    }

    fn visit_quantifier(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        println!("Found quantifier = {}", pair.as_str());
        // common preamble 
        let mut inner_pairs = pair.clone().into_inner();
        
        if let Some(first_pair) = inner_pairs.next() {

            // Initialize variables to hold closure parameters and expr
            let mut closure_param_list = None;
            let mut expr = None;

            // Loop over inner elements to find `closure_param_list` and `expr`
            for inner_pair in inner_pairs {
                match inner_pair.as_rule() {
                    Rule::closure_param_list => {
                        closure_param_list = Some(inner_pair.clone());
                    },
                    Rule::expr => {
                        expr = Some(inner_pair.clone());
                    },
                    Rule::expr_no_struct => { // treat the same as expr
                        expr = Some(inner_pair.clone());
                    },
                    _ => {}
                }
            }
            // Ensure both closure_param_list and expr were found
            if let (Some(ref closure_param_list), Some(ref expr)) = (closure_param_list, expr) {
                // Parse the closure_param_list to find variable names and types
                let mut param_map: HashMap<String, String> = HashMap::new(); // Holds variable name and type

                // Iterate over the inner pairs of closure_param_list
                let mut quant_params = closure_param_list.clone().into_inner();
                while let Some(quant_param_pair) = quant_params.next(){
                    match quant_param_pair.as_rule() {
                        Rule::param => {
                            // println!("paramPair = {:?} :: {:?}",quant_param_pair.as_rule(), quant_param_pair.as_str());
                            let mut inner_param_pairs = quant_param_pair.clone().into_inner();
                            let mut var_name = "";
                            let mut var_type = "";
                            while let Some(inner_param_pair) = inner_param_pairs.next(){
                                // println!("inner_param_pair = {:?} :: {:?}",inner_param_pair.as_rule(), inner_param_pair.as_str());
                                match inner_param_pair.as_rule() {
                                    Rule::pat_no_top_alt => {
                                            var_name = quant_param_pair.clone().as_str();
                                            // Split the string at ":"
                                            let parts: Vec<&str> = var_name.split(':').map(|part| part.trim()).collect();
                                            
                                            // Ensure there are exactly two parts
                                            if parts.len() == 2 {
                                                let key = parts[0];
                                                let value = parts[1];
                                                
                                                param_map.insert(key.to_string(),value.to_string());
                                            } else {
                                                println!("Variable format is incorrect.");
                                            }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                // Initialize a map for the variables in the expression
                let mut variables: HashMap<String, usize> = HashMap::new();
                for (key, value) in &param_map {
                    variables.insert(key.to_string(),0);
                }

                if first_pair.as_rule() == Rule::forall_str {
                    // Split implication
                    let (lhs_opt, rhs_opt) = VerusParser::split_implication(expr.as_str());
                    if let (Some(lhsExpr), Some(rhsExpr)) = (lhs_opt, rhs_opt) {
                        // println!("lhs = {:?}",lhsExpr.as_str());
                        // println!("RHS: {:?}", rhsExpr.as_str());
                        // println!("Parameters map: {:?}", param_map);
                        
                        let expr = lhsExpr.as_str();

      
                        let (all_found, missing_vars) = Self::check_missing_variables_from_expr(expr, &variables);
                    

                        if(all_found){
                            let (lower_bound, upper_bound) = Self::find_bounds(expr); // Find both bounds
                            println!("Upper bound found: {},lower bound Found {}.", upper_bound, lower_bound);
                            // Call the function to find all satisfying combinations

                            let satisfying_values = Self::find_satisfying_values(expr, &mut variables, lower_bound, upper_bound);

                            // Print out the satisfying combinations
                            // println!("Satisfying combinations:");
                            // for combination in satisfying_values.clone() {
                            //     println!("{:?}", combination);
                            // }

                            let mut expressions = Vec::new();

                            for s_val in satisfying_values {
                                // Iterate over the key-value pairs in each map
                                let mut expr_with_values = rhsExpr.as_str().to_string();
                                for (variable_name, value) in s_val {
                                    // Replace each variable in the RHS with its value
                                    let pattern = format!(r"\b{}\b", regex::escape(&variable_name));
                                    let regex = Regex::new(&pattern).unwrap();

                                    // expr_with_values = expr_with_values.replace(&variable_name, &value.to_string());
                                    expr_with_values = regex.replace_all(&expr_with_values, &value.to_string()).into_owned();

                                }
                                expressions.push(expr_with_values);
                            }
                        
                            let mut iter = expressions.clone().into_iter().peekable();

                            if let Some(first_expr) = iter.next() {
                                datum.program_mut().push_str(first_expr.as_str());
                                for expanded_rhs_expr in iter {
                                    datum.program_mut().push_str(&format!("&& {}", expanded_rhs_expr.as_str()));
                                }
                            }
                                
                        }else{ // upper bound of LHS is not concrete, so do nothing - for now! 
                            println!("No concrete Upper bound, but the passed in bound is: {}, {:?}", datum.finite_bound, missing_vars);
                            let (upper_bound, lower_bound) = Self::find_bounds(expr); // Find upper bound, lower may be defaulted
                            // Since lower bound is likely not determined, handle this case accordingly
                            println!("Lower bound found: {}, And Upper bound is not applicable in this context.", upper_bound);

                            if(missing_vars.len() == 1){ //[assumption for now]
                                let missing_var = &missing_vars[0]; // Get the missing variable
        
                                // Extract the constants in the expression for the lower bound
                                let mut lower_bound = 0;
                                let parts: Vec<&str> = expr.split_whitespace().collect(); // Assuming `expr` is available in this context
                                
                                for part in &parts {
                                    if let Ok(val) = part.parse::<usize>() {
                                        lower_bound = val;
                                        break; // Stop after finding the first constant
                                    }
                                }
                        
                                // Store the generated expressions
                                let mut generated_expressions = Vec::new();
                        
                                // Generate expressions for the missing variable
                                for val in lower_bound..=datum.finite_bound {
                                    // Generate the base expression for the missing variable
                                    let base_expression = format!("{} == {} ==> ", missing_var, val);
                                    let modified_expr = expr.replace(missing_var, &val.to_string());

                                    // Call the function to find all satisfying combinations
                                    // println!("try to find sats {:?} , {:?} , {:?} , {:?}", modified_expr, variables,upper_bound, datum.finite_bound);
                                    let satisfying_values = Self::find_satisfying_values(&modified_expr, &mut variables, upper_bound, datum.finite_bound);
                                    // println!("Satisfying combinations:");
                                    // for combination in &satisfying_values {
                                    //     println!("{:?}", combination);
                                    // }
                                    // Prepare a string to concatenate all satisfying expressions
                                    let mut rhs_expressions = Vec::new();
                        
                                    // Generate RHS expressions based on satisfying values
                                    for s_val in satisfying_values {
                                        let mut expr_with_values = rhsExpr.as_str().to_string();
                                        for (variable_name, value) in s_val {
                                            let pattern = format!(r"\b{}\b", regex::escape(&variable_name));
                                            let regex = Regex::new(&pattern).unwrap();
                                            expr_with_values = regex.replace_all(&expr_with_values, &value.to_string()).into_owned();
                                        }
                                        rhs_expressions.push(expr_with_values);
                                    }
                                    
                                    // Check if rhs_expressions is empty, and if so, add "true"
                                    if rhs_expressions.is_empty() {
                                        rhs_expressions.push("true".to_string());
                                    }
                                    
                                    // Concatenate all RHS expressions into a single string
                                    let rhs_concat = rhs_expressions.join(" && "); // Concatenate with "&&" or any desired separator
                                    
                                    
                                    // Create the final expression
                                    let final_expression = format!("({}({}))", base_expression, rhs_concat);
                                    generated_expressions.push(final_expression); // Add to the generated expressions
                                }
                        
                                // for expr in &generated_expressions {
                                //     println!("{}", expr);
                                // }
                                let mut iter = generated_expressions.clone().into_iter().peekable();

                                if let Some(first_expr) = iter.next() {
                                    datum.program_mut().push_str(first_expr.as_str());
                                    for expanded_rhs_expr in iter {
                                        datum.program_mut().push_str(&format!("&& {}", expanded_rhs_expr.as_str()));
                                    }
                                }

                                // Continue with further processing as necessary
                                // VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                            }else{

                                VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                            }
                        }
                    }
                // end forall 
                } else if first_pair.as_rule() == Rule::exists_str {
                    
                    // println!("found exists expr  {}", first_pair);
                    // println!("expr and closure = {:?} :: {:?}",expr.as_str(), closure_param_list.as_str());
                    let satisfying_values_exists = Self::find_satisfying_values_exists(&mut variables, datum.finite_bound);

                    let mut expressions = Vec::new();

                    for s_val in satisfying_values_exists {
                        // Iterate over the key-value pairs in each map
                        let mut expr_with_values = expr.as_str().to_string();
                        for (variable_name, value) in s_val {
                            // Create a regex pattern for exact match of the variable name
                            let pattern = format!(r"\b{}\b", regex::escape(&variable_name));
                            let regex = Regex::new(&pattern).unwrap();

                            // Replace only exact matches in the expression
                            expr_with_values = regex.replace_all(&expr_with_values, &value.to_string()).into_owned();
                        }
                        expressions.push(expr_with_values);
                    }

                    let mut iter = expressions.clone().into_iter().peekable();
                    if let Some(first_expr) = iter.next() {
                        datum.program_mut().push_str(&format!("({})",  first_expr.as_str()));
                        for expanded_expr in iter {
                            datum.program_mut().push_str(&format!("|| ({})",  expanded_expr.as_str()));

                        }
                    }
                }// end exists 
            }
        }
    }

// -------------------------    
// ---- HELPER FUNCTIONS ---
// -------------------------    
    fn find_bounds(expr: &str) -> (usize, usize) {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        let mut lower_bound: Option<usize> = None; // Use Option to handle uninitialized state
        let mut upper_bound: Option<usize> = None;

        // Iterate over the parts to find numeric literals
        for part in &parts {
            if let Ok(val) = part.parse::<usize>() {
                // Update the lower and upper bounds accordingly
                if lower_bound.is_none() || val < lower_bound.unwrap() {
                    lower_bound = Some(val);
                }
                if upper_bound.is_none() || val > upper_bound.unwrap() {
                    upper_bound = Some(val);
                }
            }
        }

        // Return defaults if bounds were never set
        (
            lower_bound.unwrap_or(0), // Default to 0 if no lower bound was found
            upper_bound.unwrap_or(usize::MAX), // Default to max value if no upper bound was found
        )
    }


    fn find_satisfying_values_exists(
        variables: &mut HashMap<String, usize>,
        bound: usize,
    ) -> Vec<HashMap<String, usize>> {
        let mut satisfying_combinations = Vec::new();

        // Get the variable names
        let variable_names: Vec<String> = variables.keys().cloned().collect();

        // Generate all combinations within the range for each variable
        let mut current_values = vec![0; variable_names.len()];

        loop {
            // Set each variable in the HashMap to its current value in the combination
            for (i, var_name) in variable_names.iter().enumerate() {
                variables.insert(var_name.clone(), current_values[i]);
            }

            // Add the current combination to the satisfying_combinations list
            satisfying_combinations.push(variables.clone());

            // Move to the next combination of values
            let mut idx = 0;
            while idx < current_values.len() {
                if current_values[idx] < bound {
                    current_values[idx] += 1;
                    break;
                } else {
                    current_values[idx] = 0;
                    idx += 1;
                }
            }

            if idx == current_values.len() {
                break; // Exit the loop when all combinations have been generated
            }
        }

        satisfying_combinations
    }


    fn check_missing_variables_from_expr(expr: &str, variables: &HashMap<String, usize>) -> (bool, Vec<String>) {
        // Split the expression by whitespace
        let parts: Vec<&str> = expr.split_whitespace().collect();

        // Operators to ignore
        let operators = ["<", "<=", ">", ">=", "==", "!=", "&&", "||"];
        let mut missing_variables = Vec::new(); // Vector to hold missing variable names

        for part in parts {
            // Skip operators and numeric literals
            if operators.contains(&part) || part.parse::<usize>().is_ok() || part == "bound" {
                continue;
            }

            // Check if `part` is a variable not found in `variables`
            if !variables.contains_key(part) {
                // If it's missing, add to the list of missing variables
                missing_variables.push(part.to_string());
            }
        }

        // Return a tuple: (true if no missing variables, false otherwise, and the list of missing variables)
        (missing_variables.is_empty(), missing_variables)
    }

    //currently supports expressions of the following form:
    // Constant < x < Bound
    // Constant < x < y < Bound
    // With comparison operators {< , <= , >, >=}
    fn evalForallBounds(expr: &str, variables: &HashMap<String, usize>, lower_bound: usize, upper_bound: usize) -> bool {
        let parts: Vec<&str> = expr.split_whitespace().collect();

        // Start with the first value or variable in the chain
        let mut last_value = match parts[0].parse::<usize>() {
            Ok(val) => val,                               // A numeric literal
            Err(_) => *variables.get(parts[0]).unwrap_or(&0), // Variable from param_map
        };

        let mut idx = 1;
        while idx < parts.len() - 1 {
            // Check for valid operator
            let operator = parts[idx];
            let next_value = match parts[idx + 1] {
                var_or_val => match var_or_val.parse::<usize>() {
                    Ok(val) => val, // Numeric literal
                    Err(_) => match variables.get(var_or_val) {
                        Some(&val) => val, // Variable from param_map
                        None => {
                            return false; // Handle the missing variable case
                        }
                    },
                },
            };

            // Compare based on the operator
            let comparison_holds = match operator {
                "<" => last_value < next_value,
                "<=" => last_value <= next_value,
                ">" => last_value > next_value,
                ">=" => last_value >= next_value,
                _ => {
                    println!("Unexpected operator: {}", operator);
                    return false;
                }
            };

            if !comparison_holds {
                return false;
            }

            // Move to the next value in the chain
            last_value = next_value;
            idx += 2;
        }

    // Ensure last_value is within the bounds
    last_value >= lower_bound && last_value <= upper_bound
}


    fn find_satisfying_values(
        expr: &str,
        variables: &mut HashMap<String, usize>,
        lower_bound: usize,
        upper_bound: usize,
    ) -> Vec<HashMap<String, usize>> {
        let mut satisfying_combinations = Vec::new();
        let parts: Vec<&str> = expr.split_whitespace().collect();

        // Gather variable names from the expression
        let variable_names: Vec<String> = variables.keys().cloned().collect();

        // Generate all combinations within the range and evaluate them
        let mut current_values = vec![lower_bound; variable_names.len()];

        loop {
            // Set variable values in the HashMap
            for (i, var_name) in variable_names.iter().enumerate() {
                variables.insert(var_name.clone(), current_values[i]);
            }

            // Evaluate the expression with the current combination
            if Self::evalForallBounds(expr, variables, lower_bound, upper_bound) {
                satisfying_combinations.push(variables.clone());
            }

            // Increment to the next combination
            let mut idx = 0;
            while idx < current_values.len() {
                if current_values[idx] < upper_bound {
                    current_values[idx] += 1;
                    break;
                } else {
                    current_values[idx] = lower_bound;
                    idx += 1;
                }
            }

            if idx == current_values.len() {
                break; // We’ve exhausted all combinations
            }
        }

        satisfying_combinations
    }


}
