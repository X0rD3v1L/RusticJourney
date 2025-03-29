#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn apply(&self, x: i32, y: i32) -> Option<i32> {
        match self {
            Op::Add => Some(x + y),
            Op::Sub if x > y => Some(x - y),
            Op::Mul if x != 1 && y != 1 => Some(x * y),
            Op::Div if y != 1 && x % y == 0 => Some(x / y),
            _ => None,
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Val(i32),
    App(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> Option<i32> {
        match self {
            Expr::Val(n) if *n > 0 => Some(*n),
            Expr::App(op, left, right) => {
                if let (Some(x), Some(y)) = (left.eval(), right.eval()) {
                    return op.apply(x, y);
                }
                None
            }
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expr::Val(n) => n.to_string(),
            Expr::App(op, left, right) => {
                let l_str = match **left {
                    Expr::Val(_) => left.to_string(),
                    _ => format!("({})", left.to_string()),
                };
                let r_str = match **right {
                    Expr::Val(_) => right.to_string(),
                    _ => format!("({})", right.to_string()),
                };
                format!("{} {} {}", l_str, op.to_string(), r_str)
            }
        }
    }
}

fn split<T: Clone>(xs: &[T]) -> Vec<(Vec<T>, Vec<T>)> {
    let mut result = Vec::new();
    for i in 1..xs.len() {
        result.push((xs[..i].to_vec(), xs[i..].to_vec()));
    }
    result
}

fn permutations<T: Clone>(xs: &[T]) -> Vec<Vec<T>> {
    if xs.is_empty() {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    for (i, x) in xs.iter().enumerate() {
        let rest = [&xs[..i], &xs[i + 1..]].concat();
        for mut p in permutations(&rest) {
            p.insert(0, x.clone());
            result.push(p);
        }
    }
    result
}

fn choices<T: Clone>(xs: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let len = xs.len();
    for i in 0..(1 << len) {
        let subset: Vec<T> = xs
            .iter()
            .enumerate()
            .filter(|(j, _)| (i & (1 << j)) != 0)
            .map(|(_, v)| v.clone())
            .collect();
        result.extend(permutations(&subset));
    }
    result
}

fn exprs(ns: &[i32]) -> Vec<Expr> {
    if ns.is_empty() {
        return vec![];
    }
    if ns.len() == 1 {
        return vec![Expr::Val(ns[0])];
    }
    let mut result = Vec::new();
    for (ls, rs) in split(ns) {
        for l in exprs(&ls) {
            for r in exprs(&rs) {
                for &op in &[Op::Add, Op::Sub, Op::Mul, Op::Div] {
                    result.push(Expr::App(op, Box::new(l.clone()), Box::new(r.clone())));
                }
            }
        }
    }
    result
}

fn solutions(ns: &[i32], target: i32) -> Vec<Expr> {
    let mut result = Vec::new();
    for choice in choices(ns) {
        for expr in exprs(&choice) {
            if let Some(value) = expr.eval() {
                if value == target {
                    result.push(expr);
                }
            }
        }
    }
    result
}

fn main() {
    let numbers = vec![3, 6, 8, 2];
    let target = 24;
    let solutions = solutions(&numbers, target);

    for expr in &solutions {
        println!("{}", expr.to_string());
    }

    println!("Total solutions found: {}", solutions.len());
}
