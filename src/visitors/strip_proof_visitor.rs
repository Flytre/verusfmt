use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs

// Define a new struct for your custom visitor
pub struct StripProofVisitor {
    target_name: String, // Store target_name within StripProofVisitor
}

impl StripProofVisitor {
    pub fn new(target_name: String) -> Self {
        StripProofVisitor { target_name } // Return an instance of StripProofVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        // handlers.insert("assert_expr", StripProofVisitor::visit_assert_expr);
        handlers.insert("stmt", StripProofVisitor::visit_stmt_expr);
        handlers.insert("fn", StripProofVisitor::visit_function);
        handlers.insert("identifier", StripProofVisitor::visit_identifier);
        handlers.insert("loop_clause", StripProofVisitor::visit_loop_clause);
        handlers
    }
    // loop_clause

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
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        // do nothing -- i.e. remove loop clause
    }

    fn visit_identifier(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let name = pair.as_str();
        if name == datum.target_name {
            datum.program_mut().push_str(&format!("new_{} ", name));
        } else {
            datum.program_mut().push_str(&format!("{} ", name));
        }
    }

    /// Handler for the "fn" rule. This is a Core-specific handler.
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

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }
}
