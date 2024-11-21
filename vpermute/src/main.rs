use permutohedron::LexicalPermutation;

fn main() {
    let mut visitors: Vec<&str> = vec![
        "FunctionInlineVisitor",
        "LoopVisitor",
        "ModularFlattenerVisitor",
        "RangeBoundsVisitor",
        "RecursionVisitor",
        "StripProofVisitor",
        "QuantifierVisitor",
    ];

    loop {
	println!("{:?}", visitors);
	if !visitors.next_permutation() {
	    break;
	}
    }
    println!("done!");
}
