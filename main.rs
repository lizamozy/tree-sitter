use tree_sitter::{Node, Parser};
use tree_sitter_c::tree_sitter_c;

fn main() {
    // Initialize the C parser
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c());

    // The C header file to parse
    let source_code = r#"
        header.h
    "#;

    // Parse the source code
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    // Initialize a vector to store node data
    let mut nodes_data: Vec<(String, String)> = Vec::new();

    // Traverse the tree to get data from nodes
    traverse_tree(&root_node, 0, &mut nodes_data);

    // now need to move the data into structs so I can transfer it 
}

// Function to traverse the syntax tree recursively
fn traverse_tree(node: &Node, depth: usize, nodes_data: &mut Vec<(String, String)>) {
    let node_kind = node.kind().to_string();
    let node_type = node.type_().to_string();

    // Store node kind and type into the vector
    nodes_data.push((node_kind.clone(), node_type.clone()));

    // If the node has children, recursively traverse them
    for child in node.children(&node.utf8_text_callback()) {
        traverse_tree(&child, depth + 1, nodes_data);
    }
}