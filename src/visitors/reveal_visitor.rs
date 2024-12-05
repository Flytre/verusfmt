use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs

// Define a new struct for your custom visitor
pub struct RevealVisitor {
    target_name: String, // Store target_name within RevealVisitor
}

impl RevealVisitor {
    pub fn new(target_name: String) -> Self {
        RevealVisitor { target_name } // Return an instance of RevealVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("attr_core", RevealVisitor::visit_attr_core);

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

    fn visit_attr_core<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        // println!("attr = {:?}", pair.as_str());
        let mut is_opaque = false;
        let inner_pairs = pair.clone().into_inner();
        for inner_pair in inner_pairs {
            // println!("inner = {:?} {:?}", inner_pair.as_rule(), inner_pair.as_str());
            let inner_inner_pairs = inner_pair.clone().into_inner();
            for inner_inner_pair in inner_inner_pairs {
                // println!("inner_inner_pair = {:?} {:?}", inner_inner_pair.as_rule(), inner_inner_pair.as_str());
                let inner_inner_inner_pairs = inner_inner_pair.clone().into_inner();
                for inner_inner_inner_pair in inner_inner_inner_pairs {
                    // println!("inner_inner__inner_pair = {:?} {:?}", inner_inner_inner_pair.as_rule(), inner_inner_inner_pair.as_str());
                    match inner_inner_inner_pair.as_rule() {
                        Rule::path_segment => {
                            if inner_inner_inner_pair.as_str() == "opaque" {
                                is_opaque = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        //if attribute is 'opaque` -> remove the attribute.
        //[TODO] - a better approach would be to add appropriate "reveals"
        if !is_opaque {
            datum.program_mut().push_str(&format!("{} ", pair.as_str()));
        }
    }
}
