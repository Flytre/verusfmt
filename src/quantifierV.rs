

use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;
use pest::prec_climber::{PrecClimber, Assoc, Operator};
use crate::VerusParser;

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
        handlers.insert("quantifier_expr", QuantifierVisitor::visit_quantifier);
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
    
        let mut inner_pairs = pair.clone().into_inner();
        
        if let Some(first_pair) = inner_pairs.next() {
            if first_pair.as_rule() == Rule::forall_str {

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
                        _ => {}
                    }
                }
                // Ensure both closure_param_list and expr were found
                if let (Some(ref closure_param_list), Some(ref expr)) = (closure_param_list, expr) {
                    // Parse the closure_param_list to find variable names and types
                    let mut param_map: HashMap<String, String> = HashMap::new(); // Holds variable name and type
    
                    // Iterate over the inner pairs of closure_param_list
                    let mut forall_params = closure_param_list.clone().into_inner();
                    while let Some(forall_param_pair) = forall_params.next(){
                        match forall_param_pair.as_rule() {
                            Rule::param => {
                                // println!("paramPair = {:?} :: {:?}",forall_param_pair.as_rule(), forall_param_pair.as_str());
                                let mut inner_param_pairs = forall_param_pair.clone().into_inner();
                                let mut var_name = "";
                                let mut var_type = "";
                                while let Some(inner_param_pair) = inner_param_pairs.next(){
                                    // println!("inner_param_pair = {:?} :: {:?}",inner_param_pair.as_rule(), inner_param_pair.as_str());
                                    match inner_param_pair.as_rule() {
                                        Rule::pat_no_top_alt => {
                                                var_name = forall_param_pair.clone().as_str();
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

                    // Split implication
                    let (lhs_opt, rhs_opt) = VerusParser::split_implication(expr.as_str());
                    if let (Some(lhsExpr), Some(rhsExpr)) = (lhs_opt, rhs_opt) {
                        // println!("lhs = {:?}",lhsExpr.as_str());
                        // println!("RHS: {:?}", rhsExpr.as_str());
                        // println!("Parameters map: {:?}", param_map);
                        
                        let expr = lhsExpr.as_str();

                        // Initialize a map for the variables in the expression
                        let mut variables: HashMap<String, usize> = HashMap::new();
                        for (key, value) in &param_map {
                            variables.insert(key.to_string(),0);
                        }
                        let concrete_bound = Self::check_missing_variables_from_expr(lhsExpr.as_str(), &mut variables);

                        if(concrete_bound){
                            // Call the function to find all satisfying combinations
                            let satisfying_values = Self::find_satisfying_values(expr, &mut variables, datum.finite_bound);

                            // Print out the satisfying combinations
                            // println!("Satisfying combinations:");
                            // for combination in satisfying_values.clone() {
                            //     println!("{:?}", combination);
                            // }

                            let mut expressions = Vec::new();

                            for combination in satisfying_values {
                                // Iterate over the key-value pairs in each map
                                let mut expr_with_values = rhsExpr.as_str().to_string();
                                for (variable_name, value) in combination {
                                    // Replace each variable in the RHS with its value
                                    expr_with_values = expr_with_values.replace(&variable_name, &value.to_string());
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

                            
                        }else{
                            // upper bound of LHS is not concrete, so do nothing! 
                            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                        }
                    }
                }
            } // end forall 
        }
    }

fn check_missing_variables_from_expr(expr: &str, variables: &HashMap<String, usize>) -> bool {
    // Split the expression by whitespace
    let parts: Vec<&str> = expr.split_whitespace().collect();

    // Operators to ignore
    let operators = ["<", "<=", ">", ">=", "==", "!=", "&&", "||"];

    for part in parts {
        // Skip operators and numeric literals
        if operators.contains(&part) || part.parse::<usize>().is_ok() || part == "bound" {
            continue;
        }

        // Check if `part` is a variable not found in `variables`
        if !variables.contains_key(part) {
            // println!("Variable '{}' is in the expression but not in the variables map.", part);
            return false; // Return false if any variable is missing
        }
    }

    true // Return true if all variables in `expr` are found in the map
}

    //supports expressions like:
    // Constant < x < Bound
    // Constant < x < y < bound
    // With comparison operators {< , <= , >, >=}
    fn evalForallBounds(expr: &str, variables: &HashMap<String, usize>, bound: usize) -> bool {
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
            // println!("bounds check = {}", parts[idx + 1]);
            let next_value = match parts[idx + 1] {
                "bound" => bound, // Substitute bound if referenced
                var_or_val => match var_or_val.parse::<usize>() {
                    Ok(val) => val, // Numeric literal
                    Err(_) => match variables.get(var_or_val) {
                        Some(&val) => val, // Variable from param_map
                        None => {
                            // println!("Variable '{}' not found in the map.", var_or_val);
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

        true
    }

    fn find_satisfying_values(
        expr: &str,
        variables: &mut HashMap<String, usize>,
        bound: usize,
    ) -> Vec<HashMap<String, usize>> {
        let mut satisfying_combinations = Vec::new();
        let parts: Vec<&str> = expr.split_whitespace().collect();
    
        // Extract the constants in the expression for the lower bound
        let mut lower_bound = 0;
        for part in &parts {
            if let Ok(val) = part.parse::<usize>() {
                lower_bound = val;
                break;
            }
        }
    
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
            if Self::evalForallBounds(expr, variables, bound) {
                satisfying_combinations.push(variables.clone());
            }
    
            // Increment to the next combination
            let mut idx = 0;
            while idx < current_values.len() {
                if current_values[idx] < bound {
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
