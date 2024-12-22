use crate::visitors::visitor::{CoreDatum, HandlerInterface, HandlerMap, HasProgram, VerusVisitor};
use crate::Rule;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs}; // Import Pair and Pairs
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref RECURSIVE_FIELD_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref PARENT_IMPL_NAME: Mutex<Option<String>> = Mutex::new(None);
    static ref IS_VALID_VIEW_TO_SEQ: Mutex<bool> = Mutex::new(false);

    // Add global variables for enum_name and variant_field_map
    static ref ENUM_NAME: Mutex<Option<String>> = Mutex::new(None); // To store the enum name
    static ref VARIANT_FIELD_MAP: Mutex<HashMap<String, Option<Vec<String>>>> = Mutex::new(HashMap::new()); // To store variant names and field lists
}


pub struct RecursiveDatatypeVisitor {
    _target_name: String, // Store target_name within RecursiveDatatypeVisitor
}

impl RecursiveDatatypeVisitor {
    pub fn new(target_name: String) -> Self {
        RecursiveDatatypeVisitor { _target_name: target_name } // Return an instance of RecursiveDatatypeVisitor
    }

    fn create_custom_handler_map() -> HandlerMap<CoreDatum> {
        let mut handlers = HandlerMap::new();
        handlers.insert("verus_macro_use", RecursiveDatatypeVisitor::visit_verus_macro_use);
        handlers.insert("enum", RecursiveDatatypeVisitor::visit_enum);
        handlers.insert("impl", RecursiveDatatypeVisitor::visit_impl);
        handlers.insert("assoc_item_list", RecursiveDatatypeVisitor::visit_assoc_item_list);
        handlers.insert("fn", RecursiveDatatypeVisitor::visit_function);

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

    // 1) find if there is a custom-recursive datatype
    // 2) find impl (this works if view is a seq)
    // 3) finite_bound times create depth spec fn
    

    fn visit_function(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        let current_impl_parent_name = PARENT_IMPL_NAME.lock().unwrap().clone();
    
        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::name)
            .expect("Function must have a name")
            .as_str();
    
        let ret_type = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::ret_type);
    
        if let Some(_current_impl_parent_name) = current_impl_parent_name {
            if name == "view" {
                if let Some(ret_type_pair) = ret_type {
                    let ret_type_str = ret_type_pair.as_str();
                    if ret_type_str.contains("Seq<") {
                        {
                            let mut is_valid_view_to_seq = IS_VALID_VIEW_TO_SEQ.lock().unwrap();
                            *is_valid_view_to_seq = Some(true).is_some();
                        }
                    }
                }
            }
        }
    
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }
    


    fn generate_max_depth_fn(n: u32) -> String {
        // Lock the global variables
        let enum_name = ENUM_NAME.lock().unwrap();
        let variant_field_map = VARIANT_FIELD_MAP.lock().unwrap();
        let recursive_field_names = RECURSIVE_FIELD_NAMES.lock().unwrap();
    
        // Start constructing the function definition
        let mut result_string = String::new();
    
        // Function signature with dynamic depth
        result_string.push_str(&format!(
            "spec fn maxDepth_{}(&self) -> bool\n    decreases self,\n{{\n    self.view().len() <= {}\n    && match *self {{\n",
            n, n
        ));
    
        if let Some(ref enum_name) = *enum_name {
            for (variant_name, field_list) in variant_field_map.iter() {
                result_string.push_str(&format!("        {}::{}", enum_name, variant_name));
    
                if let Some(fields) = field_list {
                    // Include fields in the match arm if they exist
                    let field_bindings = fields.join(", ");
                    result_string.push_str(&format!(" {{ {} }} => ", field_bindings));
    
                    if n == 0 {
                        // Base case for maxDepth_0
                        result_string.push_str("false");
                    } else {
                        // Recursive case for maxDepth_n
                        let recursive_fields: Vec<String> = fields
                            .iter()
                            .filter(|&field| recursive_field_names.contains(field))
                            .map(|field| format!("{}.maxDepth_{}()", field, n - 1))
                            .collect();
    
                        if recursive_fields.is_empty() {
                            result_string.push_str("true");
                        } else {
                            result_string.push_str(&format!("{{ {} }}", recursive_fields.join(" && ")));
                        }
                    }
                } else {
                    // Case for variants without fields (e.g., `Nil`)
                    result_string.push_str(if n == 0 { " => false" } else { " => true" });
                }
                result_string.push_str(",\n");
            }
        }
    
        // Close the match and the function
        result_string.push_str("    }\n}\n");
    
        result_string
    }
    
    
    
    
    
    fn visit_assoc_item_list(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
       
        // Initialize a string to store the concatenated functions
        let mut all_max_depth_fns = String::new();
      
    
        // Continue with the rest of your logic
        datum.program_mut().push_str("{\n");
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        {
            let is_valid_view_to_seq = IS_VALID_VIEW_TO_SEQ.lock().unwrap();
            // Iterate from datum.finite_bound down to 0
            if *is_valid_view_to_seq {
                for n in (0..=datum.finite_bound).rev() {
                    // Generate the maxDepth function for the current depth (convert `n` to `u32`)
                    let max_depth_fn = Self::generate_max_depth_fn(n as u32);
            
                    // Concatenate the function to the result string
                    all_max_depth_fns.push_str(&max_depth_fn);
                    all_max_depth_fns.push('\n'); // Add a newline for better formatting
                }
            }
        }
            
            // Print the concatenated result
            // println!("{}", all_max_depth_fns);

        datum.program_mut().push_str(&all_max_depth_fns);
        datum.program_mut().push_str("}\n");
    }
    
    

    fn visit_impl(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {

        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::r#type)
            .expect("Function must have a name")
            .as_str();
        {
            let mut parent_name = PARENT_IMPL_NAME.lock().unwrap();
            *parent_name = Some(name.to_string());
        }

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
        {
            let mut parent_name = PARENT_IMPL_NAME.lock().unwrap();
            *parent_name = None;
        }
       
    }

    fn visit_enum(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        // Access the global variables through the Mutexes
        let mut variant_field_map = VARIANT_FIELD_MAP.lock().unwrap();
    
        // Reset enum_name for each visit
        *ENUM_NAME.lock().unwrap() = None;
    
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::name => {
                    *ENUM_NAME.lock().unwrap() = Some(inner_pair.as_str().to_string()); // Capture the enum name
                }
                Rule::variant_list => {
                    // println!(
                    //     "inner variantList = {:?} = {:?} {:?}",
                    //     ENUM_NAME.lock().unwrap(),
                    //     inner_pair.as_rule(),
                    //     inner_pair.as_str()
                    // );
                    for inner_variant_pair in inner_pair.clone().into_inner() {
                        match inner_variant_pair.as_rule() {
                            Rule::variant => {
                                let mut field_list = None;
                                let mut variant_name = None; // Keep track of the variant name
                                for v_sub in inner_variant_pair.clone().into_inner() {
                                    match v_sub.as_rule() {
                                        Rule::name => {
                                            variant_name = Some(v_sub.as_str().to_string()); // Capture the variant name
                                        }
                                        Rule::field_list => {
                                            let mut field_names: Vec<String> = Vec::new();
                                            for v_sub_inner in v_sub.clone().into_inner() {
                                                match v_sub_inner.as_rule() {
                                                    Rule::condensable_record_field_list => {
                                                        for v_sub_inner_condensed in v_sub_inner.clone().into_inner() {
                                                            match v_sub_inner_condensed.as_rule() {
                                                                Rule::record_field => {
                                                                    let fields: Vec<_> = v_sub_inner_condensed
                                                                        .clone()
                                                                        .into_inner()
                                                                        .collect();
                                                                    for v_sub_inner_field in &fields {
                                                                        if v_sub_inner_field.as_rule() == Rule::name {
                                                                            field_names.push(v_sub_inner_field.as_str().to_string());
                                                                        }
    
                                                                        // Handle recursive fields and add to RECURSIVE_FIELD_NAMES
                                                                        if v_sub_inner_field.as_rule() == Rule::r#type {
                                                                            if let Some(name) = ENUM_NAME.lock().unwrap().as_ref() {
                                                                                let field_type = v_sub_inner_field.as_str();
    
                                                                                // Check if the field type contains the enum name (indicating recursion)
                                                                                if field_type.contains(name) {
                                                                                    // This field is recursive (e.g., next: Box<List>)
                                                                                    let field_name = fields[0].as_str();
    
                                                                                    let mut global_recursive_fields = RECURSIVE_FIELD_NAMES.lock().unwrap();
                                                                                    // Prevent duplicates by checking if the field name already exists
                                                                                    if !global_recursive_fields.contains(&field_name.to_string()) {
                                                                                        global_recursive_fields.push(field_name.to_string());
                                                                                    }
                                                                                }
    
                                                                                // Preserved logic for checking types against enum name
                                                                                if v_sub_inner_field.as_str() == name
                                                                                    || v_sub_inner_field.as_str().contains(&format!("<{}>", name))
                                                                                    || v_sub_inner_field.as_str().contains(&format!("<{}<", name))
                                                                                {
                                                                                    let field_name = fields[0].as_str();
                                                                                    let mut global_recursive_fields = RECURSIVE_FIELD_NAMES.lock().unwrap();
                                                                                    // Prevent duplicates by checking if the field name already exists
                                                                                    if !global_recursive_fields.contains(&field_name.to_string()) {
                                                                                        global_recursive_fields.push(field_name.to_string());
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            // If field names are found, store them in the map
                                            if !field_names.is_empty() {
                                                field_list = Some(field_names);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
    
                                // If no field list is found, store None. Otherwise, store the field names.
                                if let Some(name) = variant_name {
                                    if field_list.is_none() {
                                        variant_field_map.insert(name, None); // Insert Nil or any variant with no field list as None
                                    } else {
                                        variant_field_map.insert(name, field_list); // Insert variant with field names
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    
        // Output the stored variant field map
        // for (name, field_list) in variant_field_map.iter() {
        //     match field_list {
        //         Some(fields) => {
        //             println!("Variant: {} => Field List: {:?}", name, fields);
        //         }
        //         None => {
        //             println!("Variant: {} => Field List: None", name);
        //         }
        //     }
        // }
    
        // Print recursive field names
        let _recursive_field_names = RECURSIVE_FIELD_NAMES.lock().unwrap();
        // for field_name in recursive_field_names.iter() {
        //     println!("Recursive field: {}", field_name);
        // }
        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);
    }
    
    
    

    fn visit_verus_macro_use(
        datum: &mut CoreDatum,
        pair: Pair<Rule>,
        handlers: &dyn HandlerInterface<CoreDatum>,
    ) {
        datum.program_mut().push_str("verus!{\n");

        VerusVisitor::visit_all(datum, pair.into_inner(), handlers);

        datum.program_mut().push_str("}");
    }

    
    
}
