use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use crate::VerusParser;
use pest::iterators::{Pair, Pairs};
use crate::ParseAndFormatError;
use pest::Parser;

pub struct LoopVisitor {
    _target_name: String,
}

impl LoopVisitor {
    pub fn new(target_name: String) -> Self {
        LoopVisitor { _target_name: target_name }
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("while_expr", LoopVisitor::visit_while_expr);
        handlers.insert("for_expr", LoopVisitor::visit_for_expr);
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

    fn visit_for_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut loop_var = None;
        let mut loop_range = None;
        let mut loop_clause = None;
        let mut fn_block_expr = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::pat => {
                    loop_var = Some(inner_pair.clone());
                }
                Rule::expr_no_struct => {
                    loop_range = Some(inner_pair.clone());
                }
                Rule::loop_clause => {
                    loop_clause = Some(inner_pair.clone());
                }
                Rule::fn_block_expr => {
                    fn_block_expr = Some(inner_pair.clone());
                }
                _ => {}
            }
        }

        // Handle Loop Invariants
        let invariant_to_assertions = Self::extract_invariant_assertions(loop_clause);

        if let (Some(ref loop_range), Some(ref loop_var), Some(fn_block_expr)) =
            (loop_range, loop_var, fn_block_expr)
        {
            let range_str = loop_range.as_str();
            if let Some(split_index) = range_str.find("..") {
                let lhs_str = &range_str[..split_index];
                let rhs_str = &range_str[split_index + 2..]; // Skip the ".."

                let lhs = lhs_str.trim();
                let rhs = rhs_str.trim();

                // println!("LHS: {:?}", lhs);
                // println!("RHS: {:?}", rhs);

                let lhs_val = VerusParser::parse(Rule::int_number, lhs)
                    .map_err(ParseAndFormatError::from);

                let mut loop_var_assign_str: Option<String> = None;
                match lhs_val {
                    Ok(mut parsed) => {
                        if let Some(lhs_val) = parsed.next() {
                            loop_var_assign_str = Some(format!("let mut {} = {};", loop_var.as_str(), lhs_val.as_str()));
                        } else {
                            println!("Failed to parse an integer from LHS");
                        }
                    }
                    Err(_e) => {
                        loop_var_assign_str = Some(format!("let mut {} = {};", loop_var.as_str(), lhs));
                    }
                }

                if let Some(ref loop_var_assign_str) = loop_var_assign_str {
                    let full_fn_block_expr_with_counter = if fn_block_expr.as_str().ends_with('}') {
                        format!("{}{} = {} + 1;\n}}", &fn_block_expr.as_str()[..fn_block_expr.as_str().len() - 1], loop_var.as_str(), loop_var.as_str())
                    } else {
                        format!("{}\n{} = {} + 1;", fn_block_expr.as_str(), loop_var.as_str(), loop_var.as_str())
                    };
                    let full_block = Self::add_loop_inv_as_assertions(&full_fn_block_expr_with_counter, invariant_to_assertions);
                    let if_cond_str = format!("if {} < {} {}", loop_var.as_str(), rhs, full_block);
                    let if_cond = VerusParser::str_to_expr(&if_cond_str);

                    datum.program_mut().push_str(&loop_var_assign_str);
                    if let Some(pair) = if_cond {
                        for _ in 0..datum.finite_bound {
                            datum.program_mut().push_str(pair.as_str());
                        }
                    }
                }
            } else {
                println!("Invalid range format: {:?}", range_str);
            }
        }
    }

    fn visit_while_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let mut condition = None;
        let mut loop_clause = None;
        let mut fn_block_expr = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::condition => {
                    condition = Some(inner_pair.clone());
                }
                Rule::loop_clause => {
                    loop_clause = Some(inner_pair.clone());
                }
                Rule::fn_block_expr => {
                    fn_block_expr = Some(inner_pair.clone());
                }
                _ => {}
            }
        }

        let invariant_to_assertions = Self::extract_invariant_assertions(loop_clause);

        if let (Some(ref condition), Some(ref fn_block_expr)) = (condition, fn_block_expr) {
            let full_block =
                Self::add_loop_inv_as_assertions(fn_block_expr.as_str(), invariant_to_assertions);
            let if_cond_str = format!("if {} {}", condition.as_str(), full_block);
            let if_cond = VerusParser::str_to_expr(&if_cond_str);
            if let Some(pair) = if_cond {
                for _ in 0..datum.finite_bound {
                    datum.program_mut().push_str(pair.as_str());
                }
            }
        }
    }

    fn add_loop_inv_as_assertions(fn_str: &str, invariant_assertions: Vec<String>) -> String {
        let stripped_fn_str = fn_str
            .trim_start_matches("{")
            .trim_start_matches("\n")
            .trim_end_matches("}")
            .trim_end_matches("\n")
            .trim();

        let assertions_combined = invariant_assertions.join("\n");

        let final_combined_str = format!(
            "{{\n{}\n{}\n{}\n}}",
            assertions_combined, stripped_fn_str, assertions_combined
        );

        final_combined_str
    }

    fn extract_invariant_assertions(loop_clause: Option<Pair<Rule>>) -> Vec<String> {
        let mut invariant_to_assertions = Vec::new();
    
        if let Some(ref loop_clause) = loop_clause {
            let invariant_clauses = loop_clause.clone().into_inner();
            for inner_invariant_clause in invariant_clauses {
                let invariant_body = inner_invariant_clause.clone().into_inner();
                for inner_pair in invariant_body {
                    match inner_pair.as_rule() {
                        Rule::comma_delimited_exprs_for_verus_clauses => {
                            let sub_expressions = inner_pair.clone().into_inner();
                            for sub_expr in sub_expressions {
                                let invariant_to_assertion_str =
                                    format!("assert({});", sub_expr.as_str());
                                invariant_to_assertions.push(invariant_to_assertion_str);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    
        invariant_to_assertions
    }
}
