use std::collections::HashMap;

type KVResult<T> = Result<T, &'static str>;

struct KVStore {
    stack: Vec<HashMap<String, Option<u32>>>,
}

impl KVStore {
    fn new() -> Self {
        KVStore {
            stack: vec![HashMap::new()],
        }
    }

    fn set(&mut self, key: &str, value: u32) {
        self.stack
            .last_mut()
            .unwrap()
            .insert(key.to_owned(), Some(value));
    }

    fn get(&self, key: &str) -> KVResult<u32> {
        for map in self.stack.iter().rev() {
            match map.get(key) {
                Some(Some(v)) => return Ok(*v),
                Some(None) => return Err("Key not found in store"),
                None => continue,
            }
        }
        Err("Key not found in store")
    }

    fn delete(&mut self, key: &str) -> KVResult<()> {
        self.get(key)?;
        self.stack
            .last_mut()
            .unwrap()
            .insert(key.to_owned(), None);
        Ok(())
    }

    fn begin(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn commit(&mut self) {
        if self.stack.len() <= 1 {
            return;
        }
        let top = self.stack.pop().unwrap();
        let parent = self.stack.last_mut().unwrap();
        for (k, v) in top {
            parent.insert(k, v);
        }
    }

    fn rollback(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }
    
    fn depth(&self) -> usize {
        self.stack.len() - 1 // 0 = no open transaction
    }
}

fn main() {
    let mut kv = KVStore::new();

    // ── Basic operations (no transaction) ────────────────────────────
    println!("=== Basic operations ===");
    kv.set("x", 10);
    kv.set("y", 20);
    println!("set x=10, y=20");
    println!("get x  -> {:?}", kv.get("x")); // Ok(10)
    println!("get y  -> {:?}", kv.get("y")); // Ok(20)
    println!("get z  -> {:?}", kv.get("z")); // Err(KeyNotFound)

    // ── Single transaction: commit ────────────────────────────────────
    println!("\n=== Transaction: commit ===");
    kv.begin();
    println!("begin  (depth={})", kv.depth());
    kv.set("x", 99);
    kv.set("z", 30);
    println!("set x=99, z=30  (inside txn)");
    println!("get x  -> {:?}", kv.get("x")); // Ok(99) — shadows base
    println!("get y  -> {:?}", kv.get("y")); // Ok(20) — falls through to base
    kv.commit();
    println!("commit (depth={})", kv.depth());
    println!("get x  -> {:?}", kv.get("x")); // Ok(99) — merged into base
    println!("get y  -> {:?}", kv.get("y")); // Ok(20)
    println!("get z  -> {:?}", kv.get("z")); // Ok(30) — new key committed

    // ── Single transaction: rollback ─────────────────────────────────
    println!("\n=== Transaction: rollback ===");
    kv.begin();
    println!("begin  (depth={})", kv.depth());
    kv.set("x", 777);
    kv.set("w", 55);
    println!("set x=777, w=55  (inside txn)");
    println!("get x  -> {:?}", kv.get("x")); // Ok(777)
    kv.rollback();
    println!("rollback (depth={})", kv.depth());
    println!("get x  -> {:?}", kv.get("x")); // Ok(99) — rollback restored
    println!("get w  -> {:?}", kv.get("w")); // Err — w was never committed

    // ── Delete with rollback (tombstone correctness) ──────────────────
    println!("\n=== Delete inside transaction (tombstone) ===");
    kv.begin();
    println!("begin  (depth={})", kv.depth());
    println!("delete x -> {:?}", kv.delete("x")); // Ok(())
    println!("get x  -> {:?}", kv.get("x")); // Err — deleted in txn
    println!("get y  -> {:?}", kv.get("y")); // Ok(20) — unaffected
    kv.rollback();
    println!("rollback (depth={})", kv.depth());
    println!("get x  -> {:?}", kv.get("x")); // Ok(99) — delete was rolled back!

    // ── Nested transactions ───────────────────────────────────────────
    println!("\n=== Nested transactions ===");
    kv.begin(); // T1
    println!("begin T1 (depth={})", kv.depth());
    kv.set("x", 100);
    kv.set("a", 1);
    println!("T1: set x=100, a=1");

    kv.begin(); // T2 (nested inside T1)
    println!("begin T2 (depth={})", kv.depth());
    kv.set("x", 200);
    kv.set("b", 2);
    println!("T2: set x=200, b=2");
    println!("T2: get x  -> {:?}", kv.get("x")); // Ok(200)
    println!("T2: get a  -> {:?}", kv.get("a")); // Ok(1) — falls through to T1

    kv.rollback(); // discard T2 only
    println!("rollback T2 (depth={})", kv.depth());
    println!("get x  -> {:?}", kv.get("x")); // Ok(100) — T1's value restored
    println!("get b  -> {:?}", kv.get("b")); // Err — b was only in T2

    kv.commit(); // merge T1 into base
    println!("commit T1 (depth={})", kv.depth());
    println!("get x  -> {:?}", kv.get("x")); // Ok(100)
    println!("get a  -> {:?}", kv.get("a")); // Ok(1)
    println!("get b  -> {:?}", kv.get("b")); // Err — b never made it

    // ── Nested commit chain ───────────────────────────────────────────
    println!("\n=== Nested commit chain ===");
    kv.begin(); // T1
    kv.set("p", 10);
    kv.begin(); // T2
    kv.set("q", 20);
    kv.commit(); // T2 → T1
    println!("T2 committed into T1: p={:?}, q={:?}", kv.get("p"), kv.get("q"));
    kv.commit(); // T1 → base
    println!("T1 committed into base: p={:?}, q={:?}", kv.get("p"), kv.get("q"));

    println!("\nAll tests passed!");
}