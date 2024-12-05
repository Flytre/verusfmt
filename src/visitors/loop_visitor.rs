use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use crate::VerusParser;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub struct LoopVisitor {
    target_name: String,
}

impl LoopVisitor {
    pub fn new(target_name: String) -> Self {
        LoopVisitor { target_name }
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("while_expr", LoopVisitor::visit_while_expr); //only handles while loops for now
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

        // Handle Loop Invariants
        let mut invariant_to_assertions = Vec::new();

        if let Some(ref loop_clause) = loop_clause {
            let invariant_clauses = loop_clause.clone().into_inner();
            for inner_invariant_clause in invariant_clauses {
                let invariant_body = inner_invariant_clause.clone().into_inner();
                for inner_pair in invariant_body {
                    // println!("inner pair = {:?} :: {:?}", inner_pair.as_rule(), inner_pair.as_str());
                    match inner_pair.as_rule() {
                        Rule::comma_delimited_exprs_for_verus_clauses => {
                            let sub_expressions = inner_pair.clone().into_inner();
                            for sub_expr in sub_expressions {
                                // Create the assertion string directly
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

        if let (Some(ref condition), Some(ref fn_block_expr)) = (condition, fn_block_expr) {
            // println!("fn_block_expr = {:?} :: {:?}", fn_block_expr.as_rule(), fn_block_expr.as_str());
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
        // Strip the braces and newlines
        let stripped_fn_str = fn_str
            .trim_start_matches("{")
            .trim_start_matches("\n")
            .trim_end_matches("}")
            .trim_end_matches("\n")
            .trim();

        // Concatenate all assertions into a single string with newline separation
        let assertions_combined = invariant_assertions.join("\n");

        // Create the final combined string in the specified format
        let final_combined_str = format!(
            "{{\n{}\n{}\n{}\n}}",
            assertions_combined, stripped_fn_str, assertions_combined
        );

        final_combined_str
    }
}
