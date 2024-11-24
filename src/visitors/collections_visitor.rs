use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitors::visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::{Mutex};
use crate::VerusParser;

lazy_static! {
    static ref PARENT_FUNCTION_NAME: Mutex<String> = Mutex::new(String::new());
    static ref PARENT_FUNCTION_PARAM_MAP: Mutex<HashMap<String, HashMap<String,String>>> = Mutex::new(HashMap::new());
    static ref PARENT_FUNCTION_LET_TYPE_MAP: Mutex<HashMap<String, HashMap<String,String>>> = Mutex::new(HashMap::new());

}
// Define a new struct for your custom visitor
pub struct CollectionsVisitor {
    target_name: String, // Store target_name within CollectionsVisitor
}

impl CollectionsVisitor {
    pub fn new(target_name: String) -> Self {
        CollectionsVisitor { target_name } // Return an instance of CollectionsVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("expr", CollectionsVisitor::visit_expr);
        handlers.insert("fn", CollectionsVisitor::visit_function);
        handlers.insert("let_stmt", CollectionsVisitor::visit_let_stmt);


        handlers
    }

    pub fn visit_all(&self, datum: &mut CoreDatum, pairs: Pairs<Rule>) {
        let handler_map = Self::create_custom_handler_map();
        VerusVisitor::visit_all(datum, pairs, &handler_map as &dyn HandlerInterface<CoreDatum>);
    }

    fn visit_let_stmt(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {    
        let mut inner_pairs = pair.clone().into_inner();

        let mut receiving_var: Option<String> = None;
        let mut type_str: Option<String> = None;
        let mut let_param_map = HashMap::new();

        while let Some(inner_pair) = inner_pairs.next() {
    
            match inner_pair.as_rule() {
                Rule::pat => {
                    let mut pat_pair = inner_pair.clone();
                    receiving_var = Some(pat_pair.as_str().to_string());
                },
                Rule::r#type => {
                    let mut type_pair = inner_pair.clone();
                    type_str = Some(type_pair.as_str().to_string());
                },
                _ => {}
            }
            
        }
        if let (Some(receiving_var), Some(type_str)) = (receiving_var.clone(), type_str.clone()) {
            //   println!("Let Inner = {:?} {:?}", receiving_var.as_str(), type_str.as_str());
              let_param_map.insert(receiving_var, type_str);
        }
        let parent_name = PARENT_FUNCTION_NAME.lock().unwrap().clone();

        {
            let mut parent_let_param_map = PARENT_FUNCTION_LET_TYPE_MAP.lock().unwrap();
            parent_let_param_map
                .entry(parent_name.clone())
                .and_modify(|existing_map| {
                    existing_map.extend(let_param_map.clone()); // Add or overwrite existing keys
                })
                .or_insert_with(|| let_param_map.clone()); // Insert new entry if not present
        }
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut param_list = None;
        let mut param_map = HashMap::new();
    
        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::name)
            .expect("Function must have a name")
            .as_str();
    
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::param_list => {
                    param_list = Some(inner_pair.clone());
                }
                _ => {}
            }
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
        {
            let mut parent_param_map = PARENT_FUNCTION_PARAM_MAP.lock().unwrap();
            parent_param_map.insert(name.to_string(),param_map.clone());
        } 
        // for (param, param_type) in &param_map {
        //     println!("Parameter: {}, Type: {}", param, param_type);
        // }
    
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
        let mut isSeqEqualExpr = false;
        let parent_name = PARENT_FUNCTION_NAME.lock().unwrap().clone();
        
        // Retrieve `parent_params`
        let parent_params = {
            let parent_map = PARENT_FUNCTION_PARAM_MAP.lock().unwrap().clone();
            parent_map.get(&parent_name).cloned()
        };
    
        // Retrieve `parent_let_params`
        let parent_let_params = {
            let parent_let_map = PARENT_FUNCTION_LET_TYPE_MAP.lock().unwrap().clone();
            parent_let_map.get(&parent_name).cloned()
        };
    
        if pair.as_str().contains("=~=") {
            let mut inner_pairs = pair.clone().into_inner();
            for inner_pair in inner_pairs {
                match inner_pair.as_rule() {
                    Rule::bin_expr_ops => {
                        if inner_pair.as_str() == "=~=" {
                            isSeqEqualExpr = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    
        if isSeqEqualExpr {
            println!("found seq deep equiv {:?}", pair.as_str());
    
            let full_expr = pair.as_str();
            if let Some(index) = full_expr.find("=~=") {
                let left_part = full_expr[..index].trim();
                let right_part = full_expr[index + 3..].trim();
    
                println!("Left part: {:?}", left_part);
                println!("Right part: {:?}", right_part);
    
                // Numerical types for validation
                let numerical_types = vec![
                    "int", "nat", "usize", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64",
                ];
    
                // Helper function to check if a variable is a Seq<numeric type>
                let is_seq_of_numeric = |var_name: &str, params_map: &HashMap<String, String>| -> bool {
                    if let Some(var_type) = params_map.get(var_name) {
                        let var_type_trimmed = var_type.trim(); // Trim whitespace
                        if let Some(inner_type) = var_type_trimmed
                            .strip_prefix("Seq<")
                            .and_then(|v| v.strip_suffix('>'))
                        {
                            return numerical_types.contains(&inner_type);
                        }
                    }
                    false
                };
                
    
                // Check if `left_part` and `right_part` exist in `parent_params` or `parent_let_params`
                let left_is_seq = parent_params
                    .as_ref()
                    .map_or(false, |params| is_seq_of_numeric(left_part, params))
                    || parent_let_params
                        .as_ref()
                        .map_or(false, |params| is_seq_of_numeric(left_part, params));
    
                let right_is_seq = parent_params
                    .as_ref()
                    .map_or(false, |params| is_seq_of_numeric(right_part, params))
                    || parent_let_params
                        .as_ref()
                        .map_or(false, |params| is_seq_of_numeric(right_part, params));
    
                if left_is_seq && right_is_seq {
                    // Both left_part and right_part are valid Seq<numeric type>
                    let mut conditions = Vec::new();
                    for i in 0..datum.finite_bound {
                        let condition = format!(
                            "{}.len() >= {} ==> {}[{}] == {}[{}]",
                            left_part, i, left_part, i, right_part, i
                        );
                        conditions.push(condition);
                    }
    
                    // Append the conditions to the program
                    datum
                        .program_mut()
                        .push_str(&format!("({}.len() == {}.len()) &&", left_part, right_part));
                    for (i, condition) in conditions.iter().enumerate() {
                        if i == conditions.len() - 1 {
                            datum.program_mut().push_str(&format!("({})", condition));
                        } else {
                            datum.program_mut().push_str(&format!("({}) &&", condition));
                        }
                    }
                } else {
                    println!(
                        "Either left part ({}) or right part ({}) is not a Seq<numeric type>",
                        left_part, right_part
                    );
                    VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                }
            }
        } else {
            VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        }
    }
    
    


}
