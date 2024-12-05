use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use crate::{visitors::visitor::{CoreDatum, HasProgram, HandlerInterface, HandlerMap, VerusVisitor}};
use regex::Regex;


// Define a new struct for your custom visitor
pub struct LambdaVisitor {
    target_name: String, // Store target_name within LambdaVisitor
}

impl LambdaVisitor {
    pub fn new(target_name: String) -> Self {
        LambdaVisitor { target_name } // Return an instance of LambdaVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        // handlers.insert("closure_expr", LambdaVisitor::visit_closure_expr);
        handlers.insert("let_stmt", LambdaVisitor::visit_let_stmt);

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
        //------
        // ASSUME -> type of let is explicit
            // let s2: Seq<int> = 
            // (vs) let s2 = 
        //------

        let mut inner_pairs = pair.clone().into_inner();
        let mut let_expr: Option<Pair<Rule>> = None; 
        let mut closure_expr: Option<Pair<Rule>> = None; 
        let mut filter_var_name: Option<String> = None;
        let mut assign_var_name: Option<String> = None;
        let mut is_seq_type: bool = false;
        
        while let Some(inner_pair) = inner_pairs.next() {
            match inner_pair.as_rule() {
                Rule::expr => {
                    let_expr = Some(inner_pair); 
                    break;
                },
                Rule::r#type  => {
                    if inner_pair.as_str().contains("Seq") {
                        is_seq_type = true;
                    }
                },
                Rule::pat  => {
                    assign_var_name = Some(inner_pair.as_str().to_string());
                },
                _ => {}
            }
        }
        if let Some(let_expr) = let_expr {
            if let_expr.as_str().contains("filter(") {
                // for now only consider "filter"
                //search expr inners 
                let mut inner_let_expr_pairs = let_expr.clone().into_inner();
                while let Some(inner_let_expr_pair) = inner_let_expr_pairs.next() {
                    // println!("inner let expr pair = {:?} :: {:?}", inner_let_expr_pair.as_rule(), inner_let_expr_pair.as_str());
                    match inner_let_expr_pair.as_rule() {
                        Rule::arg_list => {
                            let mut inner_arg_list_pairs = inner_let_expr_pair.clone().into_inner();
                            while let Some(inner_arg_list_pair) = inner_arg_list_pairs.next() {
                                // println!("inner inner_arg_list_pair = {:?} :: {:?}", inner_arg_list_pair.as_rule(), inner_arg_list_pair.as_str());
                                match inner_arg_list_pair.as_rule() {
                                    Rule::comma_delimited_exprs => {
                                        let mut inner_comma_delimited_exprs_pairs = inner_arg_list_pair.clone().into_inner();
                                        while let Some(inner_comma_delimited_exprs_pair) = inner_comma_delimited_exprs_pairs.next() {
                                            // println!("inner inner_inner_comma_delimited_exprs_pair_pair = {:?} :: {:?}", inner_comma_delimited_exprs_pair.as_rule(), inner_comma_delimited_exprs_pair.as_str());
                                            match inner_comma_delimited_exprs_pair.as_rule() {
                                                Rule::expr => {
                                                    let mut inner_comma_delimited_exprs_expr_pairs = inner_comma_delimited_exprs_pair.clone().into_inner();
                                                    while let Some(inner_comma_delimited_exprs_expr_pair) = inner_comma_delimited_exprs_expr_pairs.next() {
                                                        // println!("inner inner_comma_delimited_exprs_expr_pair = {:?} :: {:?}", inner_comma_delimited_exprs_expr_pair.as_rule(), inner_comma_delimited_exprs_expr_pair.as_str());
                                                        match inner_comma_delimited_exprs_expr_pair.as_rule() {
                                                            Rule::expr_inner => {
                                                                let mut inner_comma_delimited_exprs_expr_expr_inner_pairs = inner_comma_delimited_exprs_expr_pair.clone().into_inner();
                                                                while let Some(inner_comma_delimited_exprs_expr_expr_inner_pair) = inner_comma_delimited_exprs_expr_expr_inner_pairs.next() {
                                                                    // println!("inner inner_comma_delimited_exprs_expr_expr_inner_pair = {:?} :: {:?}", inner_comma_delimited_exprs_expr_expr_inner_pair.as_rule(), inner_comma_delimited_exprs_expr_expr_inner_pair.as_str());
                                                                    match inner_comma_delimited_exprs_expr_expr_inner_pair.as_rule() {
                                                                        Rule::closure_expr => {
                                                                            closure_expr = Some(inner_comma_delimited_exprs_expr_expr_inner_pair.clone()); // Update the outer `let_expr`
                                                                            let mut inner_comma_closure_expr_pairs = inner_comma_delimited_exprs_expr_expr_inner_pair.clone().into_inner();
                                                                            while let Some(_inner_comma_closure_expr_pair) = inner_comma_closure_expr_pairs.next() {
                                                                                // println!("inner inner_comma_closure_expr_pair = {:?} :: {:?}", inner_comma_closure_expr_pair.as_rule(), inner_comma_closure_expr_pair.as_str());
                                                                                
                                                                            }
                                                                        },
                                                                        _ => {}
                                                                    }
                                                                }
                                                            },
                                                            _ => {}
                                                        }
                                                    }
                                                },
                                                _ => {}
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        },Rule::expr_inner => {
                            filter_var_name = Some(inner_let_expr_pair.as_str().to_string());
                        },
                        _ => {}
                    }
                }
                if is_seq_type { // basic type check. 
                    if let (Some(ref closure_expr), Some(ref filter_var_name)) = (closure_expr,filter_var_name) {
                        let mut filter_expr: Option<Pair<Rule>> = None; 
                        let mut closure_param_list = None;
                        // remove current let expr and replace with the following
                        let mut inner_closure_exprs = closure_expr.clone().into_inner();
                        while let Some(inner_closure_expr) = inner_closure_exprs.next() {
                            // println!("inner_closure_expr = {:?} :: {:?}", inner_closure_expr.as_rule(), inner_closure_expr.as_str());
                            match inner_closure_expr.as_rule() {
                                Rule::expr => {
                                    filter_expr = Some(inner_closure_expr); 
                                    break;
                                },
                                Rule::closure_param_list => {
                                    closure_param_list = Some(inner_closure_expr.clone());
                                },
                                _ => {}
                            }
                        }
                        
                        //construct finite replacement
                        if let (Some(ref closure_param_list), Some(ref filter_expr)) = (closure_param_list,filter_expr) {
                            // println!("--- Filter - EXPR =  ==  {:?} {:?}", filter_expr.as_str(), closure_param_list.as_str());
                            //parse closure_param_list to find which var to replace.
                            let mut closure_param_names: Vec<String> = Vec::new(); // Holds variable name and type
                            ////
                            let mut closure_param_list_params = closure_param_list.clone().into_inner();
                            while let Some(param_pair) = closure_param_list_params.next(){
                                match param_pair.as_rule() {
                                    Rule::param => {
                                        // println!("paramPair = {:?} :: {:?}",param_pair.as_rule(), param_pair.as_str());
                                        let mut inner_param_pairs = param_pair.clone().into_inner();
                                        let mut filter_var_name = "";
                                        while let Some(inner_param_pair) = inner_param_pairs.next(){
                                            // println!("inner_param_pair = {:?} :: {:?}",inner_param_pair.as_rule(), inner_param_pair.as_str());
                                            match inner_param_pair.as_rule() {
                                                Rule::pat_no_top_alt => {
                                                        filter_var_name = inner_param_pair.clone().as_str();
                                                        closure_param_names.push(filter_var_name.to_string());
               
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            ////
                            let mut expr_with_values = None;
                            for variable_name in closure_param_names {
                                // Replace each variable in the RHS with its value
                                let pattern = format!(r"\b{}\b", regex::escape(&variable_name));
                                let regex = Regex::new(&pattern).unwrap();

                                // expr_with_values = expr_with_values.replace(&variable_name, &value.to_string());
                                expr_with_values = Some(regex.replace_all(filter_expr.as_str(),format!("{}[filter_loop_var]",filter_var_name)).into_owned());
                                // println!("here dddd -- {:?}", expr_with_values);

                            }
                            if let (Some(ref expr_with_values), Some(ref assign_var_name)) = (expr_with_values,assign_var_name) {

                                let empty_initial_assignment = format!("let mut {} = seq![];", assign_var_name);
                                let loop_var = "let mut filter_loop_var:int = 0;";
                                let seq_len_condition = format!("if(filter_loop_var < {}.len())", filter_var_name);
                                let loop_condition = format!("if({})", expr_with_values.as_str());
                                let loop_inner = format!("{} = {}.push({}[filter_loop_var]);", assign_var_name,assign_var_name,filter_var_name);
                                let loop_var_inc = "filter_loop_var = filter_loop_var + 1;";
                                datum.program_mut().push_str(&format!("\n{}\n",  &empty_initial_assignment));
                                datum.program_mut().push_str(&format!("{}\n",  &loop_var));
                                for _ in 0..datum.finite_bound {
                                    datum.program_mut().push_str(&format!("{}\n",  &seq_len_condition));
                                    datum.program_mut().push_str("{");
                                    datum.program_mut().push_str(&loop_condition);
                                    datum.program_mut().push_str("{\n");
                                    datum.program_mut().push_str(&loop_inner);
                                    datum.program_mut().push_str("}\n");
                                    datum.program_mut().push_str(&loop_var_inc);
                                    datum.program_mut().push_str("}\n");

                                }
                            }

                        }
                        
                        
                    }
                }
            }else{
                VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
            }
        }

            
        
    }
}
