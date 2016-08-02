
#[derive(Debug)]
struct TreeNode<T: PartialOrd> {
    data: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

#[derive(Debug)]
struct Tree<T: PartialOrd> {
    root: Option<Box<TreeNode<T>>>
}

impl<T: PartialOrd> TreeNode<T> {
    pub fn new(val: T) -> TreeNode<T> {
        TreeNode {
            data: val,
            left: None,
            right: None
        }
    }

    pub fn insert(self, new_val: T) -> TreeNode<T> {
        if self.data > new_val {
            let node = match self.left {
                None => TreeNode::new(new_val),
                Some(node) => node.insert(new_val)
            };

            TreeNode {
                data: self.data,
                left: Some(Box::new(node)),
                right: self.right
            }
        } else {
            let node = match self.right {
                None => TreeNode::new(new_val),
                Some(node) => node.insert(new_val)
            };

            TreeNode {
                data: self.data,
                left: self.left,
                right: Some(Box::new(node))
            }
        }
    }
}

impl<T: PartialOrd> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root: None
        }
    }

    pub fn insert(self, new_val: T) -> Tree<T> {
        match self.root {
            None => Tree {
                root: Some(Box::new(TreeNode::new(new_val)))
            },
            Some(node) => Tree {
                root: Some(Box::new(node.insert(new_val)))
            }
        }
    }
}

fn main() {
    let mut tree: Tree<i32> = Tree::new();
    tree = tree.insert(10);
    tree = tree.insert(32);
    tree = tree.insert(1);
    println!("Tree: {:?}", tree);
}
