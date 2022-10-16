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
    pub fn preorder_traversal_recursive(&self, acc: &mut String) {
        acc.push(self.val);

        if let Some(left_rc) = self.left.as_ref() {
            left_rc.borrow().preorder_traversal_recursive(acc);
        }

        if let Some(right_rc) = self.right.as_ref() {
            right_rc.borrow().preorder_traversal_recursive(acc);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn preorder_traversal_recursive() {
        let mut output = String::new();
        ( 
            *create_traversal_tree()
            .unwrap()
         )
            .borrow()
            .preorder_traversal_recursive(&mut output);
        let expected = "ABXEMSWTPNrCH".to_string();
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
