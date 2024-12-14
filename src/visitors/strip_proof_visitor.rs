use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs

pub struct StripProofVisitor {
    _target_name: String, // Store target_name within StripProofVisitor
}

impl StripProofVisitor {
    pub fn new(target_name: String) -> Self {
        StripProofVisitor { _target_name: target_name } // Return an instance of StripProofVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        // handlers.insert("assert_expr", StripProofVisitor::visit_assert_expr);
        handlers.insert("stmt", StripProofVisitor::visit_stmt_expr);
        handlers.insert("fn", StripProofVisitor::visit_function);
        handlers.insert("loop_clause", StripProofVisitor::visit_loop_clause);
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

    fn visit_stmt_expr(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        // println!("AT Stmt {:?}", pair.as_str());
        let mut inner_pairs = pair.clone().into_inner();
        if let Some(first_pair) = inner_pairs.next() {
            // println!("AT Iner Stmt {:?}, {:?}", first_pair.as_str(), first_pair.as_rule());
            if first_pair.as_rule() == Rule::proof_block {
                // do nothing -- i.e. remove proof block
                let mut inner_proof_block_pairs= first_pair.clone().into_inner();
                for inner_proof_block_pair in inner_proof_block_pairs {
                    match inner_proof_block_pair.as_rule(){
                        Rule::stmt_list => {
                            datum.program_mut().push_str("{\n");
                            VerusVisitor::visit_all(datum, inner_proof_block_pair.clone().into_inner(), handlers);
                            datum.program_mut().push_str("\n}");
                        }
                        _ => {
                            VerusVisitor::default_visit(datum, inner_proof_block_pair, handlers);
                        }
                    }
                }
            } else {
                let mut inner_inner_pairs = first_pair.clone().into_inner();
                if let Some(first_inner_pair) = inner_inner_pairs.next() {
                    // println!("AT Iner  Inner Stmt {:?}, {:?}", first_inner_pair.as_str(), first_inner_pair.as_rule());
                    if first_pair.as_rule() == Rule::let_stmt {
                        VerusVisitor::visit_all(datum, first_pair.into_inner(), handlers);
                    }
                    let mut inner_inner_inner_pairs = first_inner_pair.clone().into_inner();
                    if let Some(first_inner_inner_pair) = inner_inner_inner_pairs.next() {
                        // println!("AT Iner  Inner Inner Stmt {:?}, {:?}", first_inner_inner_pair.as_str(), first_inner_inner_pair.as_rule());
                        match first_inner_inner_pair.as_rule() {
                            Rule::assert_expr => {
                                // do nothing -- i.e. remove assertion
                            }
                            _ => {
                                VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
                            }
                        }
                    }
                }
            }
        }
    }

    fn visit_loop_clause(
        _datum: &mut CoreDatum,
        _pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        // do nothing -- i.e. remove loop clause
    }


    /// Handler for the "fn" rule. This is a Core-specific handler.
    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }
}
