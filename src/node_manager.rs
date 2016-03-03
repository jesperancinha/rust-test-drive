use std;

fn max(x: i32, y: i32) -> i32 {
	if x < y { y } else { x }
}

enum Tree { 
	Empty,
	Leaf(i32),
	Node(Box<Tree>, Box<Tree>)
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
		&Tree::Empty => write!(f, "Empty: ({})", 0),
		&Tree::Leaf(i32) => write!(f, "Leaf: ({})", 1),
		&Tree::Node(ref left, ref right) => write!(f, "Node: ({})", max(depth(&left), depth(&right)))
		}
    }
}


fn depth(t: & Tree) -> i32 {
	match t  {
		&Tree::Empty => 0,
		&Tree::Leaf(i32) => 1,
		&Tree::Node(ref left, ref right) => max(depth(&left), depth(&right))
		}
}

pub fn run_nodes() {
let tree: Tree = Tree::Empty;
let x = depth(&tree);
println!("result is: {}", x);
let tree1: Tree = Tree::Leaf(2);
let y = depth(&tree1);
println!("result is: {}", y);
let tree2: Tree = Tree::Node(Box::new(Tree::Empty), Box::new(Tree::Leaf(1)));
let z = depth(&tree2);
println!("result is: {}", z);


println!("result is: {} {}", tree, depth(&tree));
println!("result is: {} {}", tree1, depth(&tree1));
println!("result is: {} {}", tree2, depth(&tree2));
}
