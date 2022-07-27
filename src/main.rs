#[derive(Debug)]
struct BinTree {
    root: Option<Box<Node>>,
}

impl BinTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: i32) {
        Node::insert(&mut self.root, value, 0)
    }

    fn show(&self) {
        if let Some(root) = &self.root {
            Node::show(root);
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    level: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, level: u32) -> Self {
        Self {
            value,
            level,
            left: None,
            right: None,
        }
    }


    #[allow(unused_must_use)]
    fn insert(node_or_none: &mut Option<Box<Node>>, value: i32, level: u32) {
        if let Some(node) = node_or_none {
            Self::insert(
                if node.value < value {
                    &mut node.right
                } else {
                    &mut node.left
                },
                value,
                node.level + 1
            )
        } else {
            node_or_none.insert(Box::new(Node::new(value, level)));
        }
    }

    fn print(&self) {
        let mut level = self.level;
        let mut indent = String::new();
        while level > 0 {
            level -= 1;
            indent.push_str("  ");
        }
        println!("{}{}", indent, self.value);
    }

    fn show(node: &Box<Node>) {
        if let Some(right) = &node.right {
            Node::show(right);
        } 
        node.print();
        if let Some(left) = &node.left {
            Node::show(left);
        }
    }
}

fn main() {
    let mut tree = BinTree::new();
    tree.insert(5);
    tree.insert(4);
    tree.insert(10);
    tree.insert(13);
    tree.insert(14);
    tree.insert(12);
    tree.insert(15);
    tree.show();
}
