/*
binary search tree: used for searching.
PROPERSTIES (also test cases):
    - no duplicate elements
    - for every node, left_child < node && right_child > node (cant be equal, no duplicates)
    - inorder traversal of bst gives elements in sorted order (increasing)
    - after deletion, that item must not be found
    - after insertion, that value must be found
OPERATIONS:
    - insertion: first, check if input data is in the tree. If not, insert
    - searching
    - deletion: this is a bit more complicated than search & insertion, because there aare possible scenarios
        * the node we want to delete is:
            1. leaf node -> simply unlink the node from parent
            2. has exactly 1 child (that child may be a sub-tree) -> child must take deleted node's place
            3. has 2 child:
                - 3.a: swap node with its max node in left sub-tree
                - 3.b: swap node with its min node in right sub-tree
CONS:
    - suppose we have a list: 100, 90, 80,75,70,60,50 if we create a binary search tree from this numbers
    any search operation would be O(n) time. Since the time complexity of search operation depends on the height of
    the tree, and this tree would add all nodes as left child of its parent, no siblings. It is like we are defining
    a linear list and doing a linear search. We cant control this behaviour, because this is the nature of binary search treee.
    But using other binary search tree types (self balancing trees), we can always guarantee logn search time. (AVL tree, Red-Black tree)
*/

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct BinarySearchTree {
    root: Option<Rc<RefCell<BinarySearchTreeNode>>>,
    len: usize,
}

#[derive(Debug)]
pub struct BinarySearchTreeNode {
    data: i32,
    // parent node could be null (for root node etc.)
    parent: Option<Weak<RefCell<BinarySearchTreeNode>>>,
    left_child: Option<Rc<RefCell<BinarySearchTreeNode>>>,
    right_child: Option<Rc<RefCell<BinarySearchTreeNode>>>,
}

impl BinarySearchTreeNode {
    pub fn new(data: i32) -> Self {
        Self {
            data,
            left_child: None,
            right_child: None,
            parent: None,
        }
    }

    pub fn with_parent(
        data: i32,
        parent: Weak<RefCell<BinarySearchTreeNode>>,
    ) -> Self {
        Self {
            data,
            parent: Some(parent),
            left_child: None,
            right_child: None,
        }
    }

    fn apply_preorder_to_node(&self, acc: &mut Vec<i32>) {

        acc.push(self.data);
        if let Some(left_child) = self.left_child.as_ref() {
            left_child.borrow().apply_preorder_to_node(acc);
        }

        if let Some(right_child) = self.right_child.as_ref() {
            right_child.borrow().apply_preorder_to_node(acc);
        }
    }

    fn apply_inorder_to_node(&self, acc: &mut Vec<i32>) {
        if let Some(left_child) = self.left_child.as_ref() {
            left_child.borrow().apply_inorder_to_node(acc);
        }

        acc.push(self.data);

        if let Some(right_child) = self.right_child.as_ref() {
            right_child.borrow().apply_inorder_to_node(acc);
        }
    }

    fn apply_postorder_to_node(&self, acc: &mut Vec<i32>) {
        if let Some(left_child) = self.left_child.as_ref() {
            left_child.borrow().apply_postorder_to_node(acc);
        }

        if let Some(right_child) = self.right_child.as_ref() {
            right_child.borrow().apply_postorder_to_node(acc);
        }

        acc.push(self.data);
    }

    

    pub fn num_of_childs(&self) -> usize {
        let mut count = 0;

        if self.left_child.is_some() {
            count += 1;
        }
        if self.right_child.is_some() {
            count += 1;
        }

        count
    }

    // finds the maximum valued node in the left subtree of self
    fn max_in_lsub(&self) -> Option<Rc<RefCell<BinarySearchTreeNode>>> {
        if self.left_child.is_some() {
            let mut current_node = self.left_child.clone();

            while let Some(node) = current_node {
                if node.borrow().right_child.is_none() {
                    return Some(node);
                }
                current_node = node.borrow().right_child.clone();
            }
            None
        } else {
            None
        }
    }

    // finds the minimum valued node in the right subtree of self
    fn min_in_rsub(&self) -> Option<Rc<RefCell<BinarySearchTreeNode>>> {
        if self.right_child.is_some() {
            let mut current_node = self.right_child.clone();

            while let Some(node) = current_node {
                if node.borrow().left_child.is_none() {
                    return Some(node);
                }
                current_node = node.borrow().left_child.clone();
            }
            None
        } else {
            None
        }
    }
}




pub enum TraversalType {
    InOrder,
    PreOrder,
    PostOrder
}

impl BinarySearchTree {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn insert(
        &mut self,
        insert_val: i32,
    ) {
        if self.root.is_none() {
            // no node in the tree, this node will be the root node
            self.root = Some(Rc::new(RefCell::new(
                BinarySearchTreeNode::new(insert_val),
            )));
        } else {
            let mut current_node_ptr = self.root.clone();

            while let Some(ref node_rc) = current_node_ptr {
                // the reason of this block expression:
                // since rust does not allow mutating a value if that value is borrowed (immutable/mutable),
                // if i try to assign current_node_ref value in the same scope with current_node
                // the code wont compile because current_node_ref is borrowed (the while let line (ref node_rc)).
                // i can assign outside of the scope because of Non LExical Lifetimes (NLL)
                // the compiler will understand that if node_rc borrow is not used after some point,
                // we can assume that borrow's lifetime is ended at that point.
                // so we can say that sometimes lifetimes are not bound by scopes.
                current_node_ptr = {
                    let mut current_node = node_rc.borrow_mut();
                    if insert_val == current_node.data {
                        // node already exists in the tree, no need to make an insertion
                        return;
                    } else if insert_val < current_node.data {
                        if current_node.left_child.is_none() {
                            // there is no left child, means we should insert the input data as
                            // left child of this node, this same control must be done for right child as well
                            // if there is a left child, move pointer to that child

                            current_node.left_child =
                                Some(Rc::new(RefCell::new(
                                    BinarySearchTreeNode::with_parent(
                                        insert_val,
                                        Rc::downgrade(node_rc),
                                    ),
                                )));
                            break;
                        }

                        current_node.left_child.clone()
                    } else {
                        if current_node.right_child.is_none() {
                            current_node.right_child =
                                Some(Rc::new(RefCell::new(
                                    BinarySearchTreeNode::with_parent(
                                        insert_val,
                                        Rc::downgrade(node_rc),
                                    ),
                                )));
                            break;
                        }

                        current_node.right_child.clone()
                    }
                };
            }
        }

        self.len += 1;
    }

    pub fn delete(
        &mut self,
        key: i32,
    ) {
        let target_rc = match self.search(key) {
            Some(target_rc) => target_rc,
            None => return,
        };

        self.delete_node(&target_rc);
    }

    fn delete_node(
        &mut self,
        target_rc: &Rc<RefCell<BinarySearchTreeNode>>,
    ) {
        // this method should remove the node, and make necesaarry bindings between parent and child (target's child)

        let (maybe_parent_weak, num_of_childs) = {
            let target_node = target_rc.borrow();
            (target_node.parent.clone(), target_node.num_of_childs())
        };

        match num_of_childs {
            0 => {
                if let Some(parent_rc) = maybe_parent_weak
                    .and_then(|parent_weak| parent_weak.upgrade())
                {
                    // target node is not the root node, and it is a leaf node in the tree
                    // so we should remove the link from its parent, and it will be dropped
                    // we need to find the direction from parent as well
                    let mut parent_node = parent_rc.borrow_mut();

                    if parent_node
                        .left_child
                        .as_ref()
                        .is_some_and(|lc| Rc::ptr_eq(lc, target_rc))
                    {
                        parent_node.left_child = None;
                    } else {
                        parent_node.right_child = None;
                    }
                } else {
                    // target node has no parent node, so it is the root node with zero childs
                    // so the tree has only the root node. Setting root node to None will be enough
                    self.root = None;
                }

                self.len -= 1;
            }
            1 => {
                let child_rc = target_rc
                    .borrow()
                    .left_child
                    .clone()
                    .or(target_rc.borrow().right_child.clone())
                    .unwrap();

                if let Some(parent_rc) = maybe_parent_weak
                    .and_then(|parent_weak| parent_weak.upgrade())
                {
                    // target is a non-root node in the tree and has 1 child.
                    // we must find the target's direction from its parent.
                    // and bind parent to grandchild in that direction and update the parent ptr
                    // of the grandchild to its new parent
                    let mut parent_node = parent_rc.borrow_mut();

                    if parent_node
                        .left_child
                        .as_ref()
                        .is_some_and(|lc| Rc::ptr_eq(lc, target_rc))
                    {
                        parent_node.left_child = Some(child_rc.clone());
                    } else {
                        parent_node.right_child = Some(child_rc.clone());
                    }

                    child_rc.borrow_mut().parent =
                        Some(Rc::downgrade(&parent_rc));
                } else {
                    // root node with 1 child. Find the child's direction, New root will be
                    // this child and child's parent will be None
                    child_rc.borrow_mut().parent = None;
                    self.root = Some(child_rc);
                }

                self.len -= 1;
            }
            2 => {
                // in this scenario, we must need to find max_lsub or min_rsub nodes.
                let max_lsub_rc = target_rc.borrow().max_in_lsub().unwrap();
                let max_lsub_data = max_lsub_rc.borrow().data;

                target_rc.borrow_mut().data = max_lsub_data;

                // remove this node, identified by Rc pointer
                self.delete_node(&max_lsub_rc);
            }
            _ => unreachable!(),
        }
    }

    pub fn search(
        &self,
        key: i32,
    ) -> Option<Rc<RefCell<BinarySearchTreeNode>>> {
        if self.root.is_none() {
            // no node in the tree
            None
        } else {
            let mut current_node_ptr = self.root.clone();

            while let Some(ref_node_rc) = current_node_ptr {
                let ref_current_node = ref_node_rc.borrow();

                if key == ref_current_node.data {
                    return Some(ref_node_rc.clone());
                } else if key < ref_current_node.data {
                    current_node_ptr = ref_current_node.left_child.clone();
                } else {
                    current_node_ptr = ref_current_node.right_child.clone();
                }
            }

            None
        }
    }

    pub fn traverse(&self, traversal_type: TraversalType) -> Option<Vec<i32>> {

        match self.root.as_ref() {
            Some(_) => {
                match traversal_type {
                    TraversalType::InOrder => self.get_inorder_vec(),
                    TraversalType::PreOrder => self.get_preorder_vec(),
                    TraversalType::PostOrder => self.get_postorder_vec(),
                }
            },
            None => None,
        }
    }

    fn get_postorder_vec(&self) -> Option<Vec<i32>> {
        let mut result_vec = Vec::with_capacity(self.get_len());

        if let Some(root_rc) = self.root.as_ref() {
            let root_node = root_rc.borrow();
            root_node.apply_postorder_to_node(&mut result_vec);
        }

        Some(result_vec)
    }

    fn get_preorder_vec(&self) -> Option<Vec<i32>> {
        let mut result_vec = Vec::with_capacity(self.get_len());

        if let Some(root_rc) = self.root.as_ref() {
            let root_node = root_rc.borrow();
            root_node.apply_preorder_to_node(&mut result_vec);
        }

        Some(result_vec)
    }

    fn get_inorder_vec(&self) -> Option<Vec<i32>> {
        let mut result_vec = Vec::with_capacity(self.get_len());

        if let Some(root_rc) = self.root.as_ref() {
            let root_node = root_rc.borrow();
            root_node.apply_inorder_to_node(&mut result_vec);
        }

        Some(result_vec)
    }

    pub fn traverse_inorder_print(
        root: Option<&Rc<RefCell<BinarySearchTreeNode>>>
    ) {
        match root {
            Some(ref node_rc) => {
                let current_node = node_rc.borrow();

                Self::traverse_inorder_print(
                    current_node.left_child.as_ref().clone(),
                );
                println!("{:?}", current_node.data);
                Self::traverse_inorder_print(
                    current_node.right_child.as_ref().clone(),
                );
            }
            None => return,
        }
    }

    pub fn traverse_preorder_print(
        root: Option<&Rc<RefCell<BinarySearchTreeNode>>>
    ) {
        match root {
            Some(ref node_rc) => {
                let current_node = node_rc.borrow();
                println!("{:?}", current_node.data);
                Self::traverse_preorder_print(
                    current_node.left_child.as_ref().clone(),
                );
                Self::traverse_preorder_print(
                    current_node.right_child.as_ref().clone(),
                );
            }
            None => return,
        }
    }

    pub fn traverse_postorder_print(
        root: Option<&Rc<RefCell<BinarySearchTreeNode>>>
    ) {
        match root {
            Some(ref node_rc) => {
                let current_node = node_rc.borrow();
                Self::traverse_postorder_print(
                    current_node.left_child.as_ref().clone(),
                );
                Self::traverse_postorder_print(
                    current_node.right_child.as_ref().clone(),
                );
                println!("{:?}", current_node.data);
            }
            None => return,
        }
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_root(&self) -> Option<&Rc<RefCell<BinarySearchTreeNode>>> {
        self.root.as_ref()
    }
    
    
}

#[cfg(test)]
mod binary_tree_tests {
    use std::collections::HashSet;

    use super::*;
    use proptest::prelude::*;

    #[test]
    fn inorder_traversal_gives_sorted_elements() {
        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = proptest::collection::vec(any::<i32>(), 1..500);


        runner.run(&strategy, |values| {

            let mut bstree = BinarySearchTree::new();

            for &value in &values {
                bstree.insert(value);
            }

            let mut seen_values = HashSet::new();
            let mut unique_values = Vec::new();

            for &value in &values {
                if seen_values.insert(value) {
                    unique_values.push(value);
                }
            }

            let maybe_inorder_vec = bstree.get_inorder_vec();
            if let Some(inorder_vec) = maybe_inorder_vec {
                let mut sorted = inorder_vec.clone();
                sorted.sort();
                prop_assert_eq!(inorder_vec, sorted);
            }

            Ok(())
        }).unwrap();
    }

    #[test]
    fn no_duplicate_elements() {
        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = proptest::collection::vec(any::<i32>(), 1..500);


        runner.run(&strategy, |values| {

            let mut bstree = BinarySearchTree::new();

            for &value in &values {
                bstree.insert(value);
            }

            let mut seen_values = HashSet::new();
            let mut unique_values = Vec::new();

            for &value in &values {
                if seen_values.insert(value) {
                    unique_values.push(value);
                }
            }


            prop_assert_eq!(
                bstree.get_len(), 
                unique_values.len(),
                "bstree get_len() must match number of unique elements"
            );


            for &unique_value in &unique_values {
                let len_before_deletion = bstree.get_len();
                bstree.delete(unique_value);

                prop_assert_eq!(bstree.get_len(), len_before_deletion - 1);

                prop_assert!(bstree.search(unique_value).is_none(), 
                    "No duplicate elements should be present"
                );
            }

            prop_assert_eq!(bstree.get_len(), 0);
            
            for &unique_value in &unique_values {
                prop_assert!(bstree.search(unique_value).is_none());
            }

            Ok(())
        }).unwrap();


    }

    #[test]
    fn inserted_node_should_be_found() {
        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = proptest::collection::vec(any::<i32>(), 1..500);

        runner
            .run(&strategy, |values| {
                // create the bst, and insert the values
                let mut bstree = BinarySearchTree::new();

                for &value in &values {
                    bstree.insert(value);
                }

                let mut seen_values = HashSet::new();
                let mut unique_values_in_tree = Vec::new();
                for &v in &values {
                    if seen_values.insert(v) {
                        unique_values_in_tree.push(v);
                    }
                }

                for &unique_value in &unique_values_in_tree {
                    prop_assert!(
                        bstree.search(unique_value).is_some(),
                        "value {} should be present in the tree",
                        unique_value
                    );
                }

                if let Some(max_value) = unique_values_in_tree.iter().max() {
                    prop_assert!(
                        bstree.search(max_value + 1).is_none(),
                        "value {} should NOT be present in the tree",
                        max_value + 1
                    );
                }

                prop_assert_eq!(bstree.get_len(), 
                    unique_values_in_tree.len(), 
                    "get_len MUST be the same len with unique_values_in_tree"
                );



                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn deleted_nodes_should_not_be_found() {
        let mut runner = proptest::test_runner::TestRunner::default();
        let strategy = proptest::collection::vec(any::<i32>(), 1..500);

        runner
            .run(&strategy, |values| {
                let mut bstree = BinarySearchTree::new();

                for &value in &values {
                    bstree.insert(value);
                }

                // create a list of unique values from tree
                let mut seen_values = HashSet::new();
                let mut unique_values_in_tree = Vec::new();
                for &v in &values {
                    if seen_values.insert(v) {
                        unique_values_in_tree.push(v);
                    }
                }

                // from unique values, select some nodes to dolete
                let nodes_to_delete: Vec<i32> = unique_values_in_tree
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, &v)| v)
                    .collect();

                for &node_to_delete in &nodes_to_delete {
                    bstree.delete(node_to_delete);
                }

                // this checks the deleted node is in the tree or not
                for &node in &nodes_to_delete {
                    prop_assert!(
                        bstree.search(node).is_none(),
                        "deleted value {} should not be found",
                        node
                    );
                }

                // find the remaining unique nodes
                let remaining_nodes_after_deletion: Vec<i32> =
                    unique_values_in_tree
                        .into_iter()
                        .filter(|node| !nodes_to_delete.contains(node))
                        .collect();

                for node in &remaining_nodes_after_deletion {
                    prop_assert!(
                        bstree.search(*node).is_some(),
                        "non deleted value {} should still be found",
                        node
                    );
                }

                prop_assert_eq!(
                    bstree.get_len(),
                    remaining_nodes_after_deletion.len(),
                    "bstree get_len() must match number of remaining unique elements"
                );

                Ok(())
            })
            .unwrap();
    }
}
