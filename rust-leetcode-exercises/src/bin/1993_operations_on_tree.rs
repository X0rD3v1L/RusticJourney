use std::cell::RefCell;

struct LockingTree {
    parent: Vec<usize>,
    children: Vec<Vec<usize>>,
    locked: Vec<RefCell<Option<i32>>>,
}

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let n = parent.len();
        let mut children = vec![vec![]; n];

        for (i, &p) in parent.iter().enumerate() {
            if p != -1 {
                children[p as usize].push(i);
            }
        }

        Self {
            parent: parent.iter().map(|&x| x as usize).collect(),
            children,
            locked: vec![RefCell::new(None); n],
        }
    }

    fn lock(&self, num: i32, user: i32) -> bool {
        let idx = num as usize;
        let mut lock = self.locked[idx].borrow_mut();
        if lock.is_none() {
            *lock = Some(user);
            return true;
        }
        false
    }

    fn unlock(&self, num: i32, user: i32) -> bool {
        let idx = num as usize;
        let mut lock = self.locked[idx].borrow_mut();
        if *lock == Some(user) {
            *lock = None;
            return true;
        }
        false
    }

    fn upgrade(&self, num: i32, user: i32) -> bool {
        let idx = num as usize;

        // 1. Node must be unlocked
        if self.locked[idx].borrow().is_some() {
            return false;
        }

        // 2. No locked ancestors
        let mut p = self.parent[idx];
        while p != usize::MAX {
            if self.locked[p].borrow().is_some() {
                return false;
            }
            p = self.parent[p];
        }

        // 3. At least one locked descendant
        let mut found = false;
        self.has_locked_descendant(idx, &mut found);
        if !found {
            return false;
        }

        self.unlock_all_descendants(idx);
        *self.locked[idx].borrow_mut() = Some(user);
        true
    }

    fn has_locked_descendant(&self, idx: usize, found: &mut bool) {
        for &child in &self.children[idx] {
            if self.locked[child].borrow().is_some() {
                *found = true;
            }
            self.has_locked_descendant(child, found);
        }
    }

    fn unlock_all_descendants(&self, idx: usize) {
        for &child in &self.children[idx] {
            *self.locked[child].borrow_mut() = None;
            self.unlock_all_descendants(child);
        }
    }
}

fn main() {
    let tree = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
    let mut tree = tree;

    assert_eq!(tree.lock(2, 2), true);
    assert_eq!(tree.unlock(2, 3), false);
    assert_eq!(tree.unlock(2, 2), true);
    assert_eq!(tree.lock(4, 5), true);
    assert_eq!(tree.upgrade(0, 1), true);
    
    assert_eq!(tree.locked[0].borrow().is_some(), true); // node 0 is now locked
    assert_eq!(tree.locked[4].borrow().is_none(), true); // node 4 is unlocked

    println!("All operations completed successfully.");
}
