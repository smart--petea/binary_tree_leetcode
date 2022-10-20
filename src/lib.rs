use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

type WrappedTreeNode = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: char,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: char) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }

  pub fn new_wrapped(val: char, left: WrappedTreeNode, right: WrappedTreeNode) -> WrappedTreeNode {
      let treeNode = TreeNode { val, left, right };

      Some(Rc::new(RefCell::new(treeNode)))
  }
}

impl TreeNode {
    pub fn preorder_traversal_recursive(node: &WrappedTreeNode, acc: &mut String) {
        if node.is_none() {
            return;
        }

        let node = (*(*node.as_ref().unwrap())).borrow();
        acc.push(node.val);
        TreeNode::preorder_traversal_recursive(&node.left, acc);
        TreeNode::preorder_traversal_recursive(&node.right, acc);
    }

    pub fn preorder_traversal_iterative(node: WrappedTreeNode, acc: &mut String) {
        let mut stack: Vec<WrappedTreeNode> = vec![node];

        while let Some(Some(rc)) = stack.pop() {
            let inner = (*rc).borrow();
            acc.push(inner.val);

            if let Some(ref right_rc) = inner.right {
                stack.push(Some(right_rc.clone()));
            }

            if let Some(ref left_rc) = inner.left {
                stack.push(Some(left_rc.clone()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preorder_traversal_recursive() {
        let mut output = String::new();
        let root = create_traversal_tree();
        TreeNode::preorder_traversal_recursive(&root, &mut output);
        let expected = "ABXEMSWTPNCH".to_string();
        assert_eq!(output, expected);
    }

    #[test]
    fn preorder_traversal_iterative() {
        let mut output = String::new();
        let root = create_traversal_tree();
        TreeNode::preorder_traversal_iterative(root, &mut output);
        let expected = "ABXEMSWTPNCH".to_string();
        assert_eq!(output, expected);
    }

    fn create_traversal_tree() -> WrappedTreeNode {
        let e = TreeNode::new_wrapped('E', None, None);
        let m = TreeNode::new_wrapped('M', None, None);
        let x = TreeNode::new_wrapped('X', e, m);
        let s = TreeNode::new_wrapped('S', None, None);
        let b = TreeNode::new_wrapped('B', x, s);
        let p = TreeNode::new_wrapped('P', None, None);
        let n = TreeNode::new_wrapped('N', None, None);
        let t = TreeNode::new_wrapped('T', p, n);
        let h = TreeNode::new_wrapped('H', None, None);
        let c = TreeNode::new_wrapped('C', h, None);
        let w = TreeNode::new_wrapped('W', t, c);
        let a = TreeNode::new_wrapped('A', b, w);

        a
    }
}
