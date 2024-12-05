use crate::Rule;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub trait HasProgram {
    fn program_mut(&mut self) -> &mut String;
}

pub trait HandlerInterface<T: HasProgram> {
    fn get_handler(&self, rule: &str) -> Option<fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)>;
}

pub struct HandlerMap<T: HasProgram> {
    handlers: HashMap<&'static str, fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)>,
}

impl<T: HasProgram> HandlerMap<T> {
    // Initialize a new handler map
    pub fn new() -> Self {
        let mut handlers: HashMap<&'static str, fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)> =
            HashMap::new();

        // Insert handlers into the map
        handlers.insert("verus_macro_use", VerusVisitor::visit_verus_macro_use);
        handlers.insert("param_list", VerusVisitor::visit_param_list);
        handlers.insert("fn_block_expr", VerusVisitor::visit_fn_block_expr);
        handlers.insert("stmt_list", VerusVisitor::visit_stmt_list);
        handlers.insert("closure_param_list", VerusVisitor::visit_closure_param_list);
        handlers.insert("generic_param_list", VerusVisitor::visit_generic_param_list); // new
        handlers.insert(
            "comma_delimited_exprs",
            VerusVisitor::visit_comma_delimited_exprs,
        );
        handlers.insert(
            "comma_delimited_exprs_for_verus_clauses",
            VerusVisitor::visit_comma_delimited_exprs_for_verus_clauses,
        );
        handlers.insert("paren_expr_inner", VerusVisitor::visit_paren_expr_inner);
        handlers.insert("arg_list", VerusVisitor::visit_arg_list);
        handlers.insert("COMMENT", VerusVisitor::visit_comment);
        handlers.insert("item_list", VerusVisitor::visit_item_list);
        handlers.insert("record_field_list", VerusVisitor::visit_record_field_list);
        handlers.insert("assoc_item_list", VerusVisitor::visit_assoc_item_list);
        handlers.insert("match_arm_list", VerusVisitor::visit_match_arm_list);
        handlers.insert(
            "tuple_struct_pat_inner",
            VerusVisitor::visit_tuple_struct_pat_inner,
        );
        handlers.insert("tuple_pat", VerusVisitor::visit_tuple_struct_pat);
        handlers.insert("match_arm_lhs", VerusVisitor::visit_match_arm_lhs);
        handlers.insert(
            "record_expr_field_list",
            VerusVisitor::visit_record_expr_field_list,
        );
        handlers.insert("trigger_str", VerusVisitor::visit_trigger_str);
        handlers.insert("generic_args", VerusVisitor::visit_generic_args);
        handlers.insert("ref_type", VerusVisitor::visit_ref_type);
        handlers.insert("use_tree_list", VerusVisitor::visit_use_tree_list);
        handlers.insert("attr_core", VerusVisitor::visit_attr_core);
        handlers.insert("variant_list", VerusVisitor::visit_variant_list);
        handlers.insert("field_list", VerusVisitor::visit_field_list);
        handlers.insert(
            "condensable_record_field_list",
            VerusVisitor::visit_condensable_record_field_list,
        );
        handlers.insert(
            "record_pat_field_list",
            VerusVisitor::visit_record_pat_field_list,
        );
        handlers.insert("tuple_field_list", VerusVisitor::visit_tuple_field_list);
        Self { handlers }
    }

    pub fn insert(
        &mut self,
        key: &'static str,
        handler: fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>),
    ) {
        self.handlers.insert(key, handler);
    }
}

impl<T: HasProgram> HandlerInterface<T> for HandlerMap<T> {
    fn get_handler(&self, rule: &str) -> Option<fn(&mut T, Pair<Rule>, &dyn HandlerInterface<T>)> {
        self.handlers.get(rule).copied() // return the function pointer
    }
}

pub struct VerusVisitor;

impl VerusVisitor {
    pub fn visit<T: HasProgram>(datum: &mut T, pair: Pair<Rule>, handlers: &dyn HandlerInterface<T>) {
        // println!("VISITING {:?} {:?}", pair.as_rule(), pair.as_str());
        let rule_name = format!("{:?}", pair.as_rule());
        if let Some(handler) = handlers.get_handler(&rule_name) {
            handler(datum, pair, handlers);
        } else {
            VerusVisitor::default_visit(datum, pair, handlers);
        }
    }

    pub fn default_visit<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        // println!("VISITING {:?} : {}", pair.as_rule(), pair.as_str());
        let inner_pairs = pair.clone().into_inner();
        if inner_pairs.clone().count() == 0 {
            datum.program_mut().push_str(&format!("{} ", pair.as_str()));
        } else {
            VerusVisitor::visit_all(datum, inner_pairs, handlers);
        }
    }
    fn visit_attr_core<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str(&format!("{} ", pair.as_str()));
    }

    fn visit_record_pat_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{ ");
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str(" }");
    }

    fn visit_tuple_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        let mut inner_pairs = pair.into_inner().peekable(); // Make the iterator peekable

        while let Some(inner_pair) = inner_pairs.next() {
            VerusVisitor::visit(datum, inner_pair, handlers);

            if inner_pairs.peek().is_some() {
                datum.program_mut().push_str(", ");
            }
        }
    }

    fn visit_condensable_record_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        let mut inner_pairs = pair.into_inner().peekable(); // Make the iterator peekable

        while let Some(inner_pair) = inner_pairs.next() {
            VerusVisitor::visit(datum, inner_pair, handlers);

            if inner_pairs.peek().is_some() {
                datum.program_mut().push_str(", ");
            }
        }
    }

    fn visit_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        let first_char = pair.as_str().chars().next();

        if let Some('{') = first_char {
            // Handle curly brace case
            datum.program_mut().push_str("{");
            for inner_pair in pair.into_inner() {
                VerusVisitor::visit(datum, inner_pair, handlers);
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str("}");
        } else if let Some('(') = first_char {
            // Handle parenthesis
            datum.program_mut().push_str("(");
            for inner_pair in pair.into_inner() {
                VerusVisitor::visit(datum, inner_pair, handlers);
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str(")");
        }
    }

    fn visit_variant_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str("}\n");
    }

    fn visit_item_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_record_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            datum.program_mut().push_str(inner_pair.as_str());
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str("}\n");
    }

    fn visit_assoc_item_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_match_arm_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        // VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_tuple_struct_pat_inner<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("(");
        // VerusVisitor::visit_all(datum, pair.into_inner(), handlers)
        let mut inner_pairs = pair.into_inner().peekable(); // Make the iterator peekable

        while let Some(inner_pair) = inner_pairs.next() {
            VerusVisitor::visit(datum, inner_pair, handlers);

            if inner_pairs.peek().is_some() {
                datum.program_mut().push_str(", ");
            }
        }

        datum.program_mut().push_str(")");
    }

    fn visit_tuple_struct_pat<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("(");

        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str(")");
    }

    fn visit_match_arm_lhs<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("=>");
    }

    fn visit_record_expr_field_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str("}\n");
    }

    fn visit_trigger_str<T: HasProgram>(
        datum: &mut T,
        _pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("[trigger]");
    }

    fn visit_generic_args<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
    }

    fn visit_ref_type<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("&");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }

    fn visit_use_tree_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{");
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str("}");
    }

    fn visit_stmt_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_verus_macro_use<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("verus!{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("}\n");
    }

    fn visit_generic_param_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("<");
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
        datum.program_mut().push_str(">");
    }

    fn visit_param_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push('(');
        let mut first = true;
        for inner_pair in pair.into_inner() {
            if !first {
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str(inner_pair.as_str());
            first = false;
        }
        datum.program_mut().push(')');
    }

    fn visit_fn_block_expr<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("\n {");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str("\n } \n");
    }

    fn visit_paren_expr_inner<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push_str("(");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push_str(")");
    }

    fn visit_closure_param_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push('|');
        let mut first = true;
        for inner_pair in pair.into_inner() {
            if !first {
                datum.program_mut().push_str(", ");
            }
            datum.program_mut().push_str(inner_pair.as_str());
            first = false;
        }
        datum.program_mut().push('|');
    }

    fn visit_comma_delimited_exprs<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for inner_pair in pair.into_inner() {
            VerusVisitor::visit(datum, inner_pair, handlers);
            datum.program_mut().push_str(", ");
        }
    }

    fn visit_comma_delimited_exprs_for_verus_clauses<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::COMMENT => {
                    // dont add "extra" comma if element is comment
                }
                _ => {
                    VerusVisitor::visit(datum, inner_pair, handlers);
                    datum.program_mut().push_str(", \n");
                }
            }
        }
    }

    fn visit_arg_list<T: HasProgram>(
        datum: &mut T,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        datum.program_mut().push('(');
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        datum.program_mut().push(')');
    }

    fn visit_comment<T: HasProgram>(
        _datum: &mut T,
        _pair: Pair<Rule>,
        _handlers: &dyn HandlerInterface<T>,
    ) {
        // Do nothing for comments
        //[TODO] : Comments in the middle of comma separated list:
        //i.e. this example causes an error
        // ensures f.len() == n,
        // f[0] == 0,  // comments in the middle of comma list still not covered
        // f[n-1] != 0,
    }

    pub fn visit_all<T: HasProgram>(
        datum: &mut T,
        pairs: Pairs<Rule>,
        handlers: &dyn HandlerInterface<T>,
    ) {
        for pair in pairs {
            VerusVisitor::visit(datum, pair, handlers);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CoreDatum {
    pub program: String,
    pub fn_map: HashMap<String, String>, // Assume names are unique for now
    pub fn_calls: HashMap<String, Vec<Vec<String>>>,
    pub target_name: String, // Add target_name
    pub finite_bound: usize,
    pub variable_map: HashMap<String, String>,
    pub variable_stack: Vec<Vec<String>>,
}

// Implement HasProgram for CoreDatum
impl HasProgram for CoreDatum {
    fn program_mut(&mut self) -> &mut String {
        &mut self.program
    }
}
