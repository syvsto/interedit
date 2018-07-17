use super::TokenizedFile;

#[derive(Debug)]
struct EvalStack {
    ops: Vec<AlgorithmOp>,
    vals: Vec<i32>,
}

impl EvalStack {
    fn new() -> Self {
        Self {
            ops: Vec::new(),
            vals: Vec::new(),
        }
    }

    fn add_op(&mut self, op: AlgorithmOp) {
        self.ops.push(op);
    }

    fn add_val(&mut self, val: i32) {
        self.vals.push(val);
    }

    fn eval_op(op: &AlgorithmOp, vals: &mut Vec<i32>) {
        match op {
            AlgorithmOp::Add => {
                let arg1 = vals.pop().unwrap();
                let arg2 = vals.pop().unwrap();
                vals.push(arg1 + arg2);
            }
            AlgorithmOp::Sub => {
                let arg1 = vals.pop().unwrap();
                let arg2 = vals.pop().unwrap();
                vals.push(arg1 - arg2);
            }
            AlgorithmOp::Mul => {
                let arg1 = vals.pop().unwrap();
                let arg2 = vals.pop().unwrap();
                vals.push(arg1 * arg2);
            }
            AlgorithmOp::Div => {
                let arg1 = vals.pop().unwrap();
                let arg2 = vals.pop().unwrap();
                vals.push(arg1 / arg2);
            }
            AlgorithmOp::None => {}
        }
    }

    fn eval(&mut self) -> EvalResult {
        for op in self.ops.iter() {
            EvalStack::eval_op(op, &mut self.vals);
        }
        if let Some(v) = self.vals.pop() {
            EvalResult::Int(v)
        } else {
            EvalResult::Error("Couldn't evaluate".to_string())
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum AlgorithmOp {
    Add,
    Sub,
    Mul,
    Div,
    None,
}

#[derive(Debug)]
pub enum EvalResult {
    Int(i32),
    Error(String),
}

impl ToString for EvalResult {
    fn to_string(&self) -> String {
        match self {
            EvalResult::Int(i) => i.to_string(),
            EvalResult::Error(_) => format!(""),
        }
    }
}

pub fn evaluate(file: &TokenizedFile) -> Result<Vec<EvalResult>, String> {
    let mut results = Vec::new();

    for f in file.algorithms.iter() {
        let mut eval_stack = EvalStack::new();
        for t in &f.ty {
            parse_algorithm(&t, file, &mut eval_stack)?;
        }
        results.push(eval_stack.eval());
    }
    results.reverse();
    Ok(results)
}

fn parse_arg(arg: &str, file: &TokenizedFile) -> i32 {
    let mut chars = arg.chars();
    if chars.next().unwrap() == ':' {
        let id_str = chars.collect::<String>();
        let ep = file.find_value_by_id(id_str.parse().unwrap()).unwrap();
        ep.val
    } else {
        arg.parse().unwrap()
    }
}

fn parse_algorithm(
    algorithm_string: &str,
    file: &TokenizedFile,
    eval_stack: &mut EvalStack,
) -> Result<(), String> {
    let mut algorithm = algorithm_string.split(' ');
    let op = parse_op(algorithm.next().unwrap());
    let vals: Vec<i32> = algorithm.map(|s| parse_arg(s, file)).collect();
    eval_stack.add_op(op);
    vals.iter().for_each(move |v| eval_stack.add_val(*v));
    Ok(())
}

fn parse_op(op_str: &str) -> AlgorithmOp {
    match op_str {
        "add" => AlgorithmOp::Add,
        "sub" => AlgorithmOp::Sub,
        "mul" => AlgorithmOp::Mul,
        "div" => AlgorithmOp::Div,
        _ => AlgorithmOp::None,
    }
}
