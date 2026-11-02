# homework 2 - my answers

**Name:** Berkay Coskun

hi, here are my answers for homework 2.

## part 1 - operators (tasks 1-5)

I used `match` for the four operators. For task 5, `?` returns `None` if the character is not an operator.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op { Add, Sub, Mul, Div }

impl Op {
    fn from_char(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' => Some(Op::Mul),
            '/' => Some(Op::Div),
            _ => None,
        }
    }

    fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    fn precedence(self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }

    fn apply(self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div if b != 0.0 => Some(a / b),
            Op::Div => None,
        }
    }

    fn evaluate_simple(a: f64, op: char, b: f64) -> Option<f64> {
        Op::from_char(op)?.apply(a, b)
    }
}
```

## part 2 - stack (tasks 6-9)

I used a `Vec<f64>` for the stack. In `apply`, I check the operation before popping anything, so the stack stays the same if it fails.

```rust
#[derive(Debug)]
struct Stack { items: Vec<f64> }

impl Stack {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn push(&mut self, v: f64) {
        self.items.push(v);
    }

    fn pop(&mut self) -> Option<f64> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&f64> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn apply(&mut self, op: Op) -> Option<()> {
        if self.len() < 2 {
            return None;
        }

        let b = self.items[self.len() - 1];
        let a = self.items[self.len() - 2];
        let result = op.apply(a, b)?;

        self.pop();
        self.pop();
        self.push(result);
        Some(())
    }
}
```

## task 10 - calculation and a few checks

I calculated `(2 + 3) * 4 - 1` using `push` and `apply`, then emptied the stack using `while let`.

```rust
fn main() {
    assert_eq!(Op::from_char('*'), Some(Op::Mul));
    assert_eq!(Op::from_char('x'), None);
    assert_eq!(Op::Div.symbol(), '/');
    assert_eq!(Op::Add.precedence(), 1);
    assert_eq!(Op::Mul.precedence(), 2);
    assert_eq!(Op::Sub.apply(5.0, 3.0), Some(2.0));
    assert_eq!(Op::Div.apply(1.0, 0.0), None);
    assert_eq!(Op::evaluate_simple(6.0, '/', 4.0), Some(1.5));
    assert_eq!(Op::evaluate_simple(1.0, '?', 2.0), None);
    assert_eq!(Op::evaluate_simple(1.0, '/', 0.0), None);

    let mut check = Stack::new();
    assert_eq!(check.peek(), None);
    check.push(1.0);
    check.push(2.0);
    assert_eq!(check.peek(), Some(&2.0));
    assert_eq!(check.len(), 2);
    assert_eq!(check.pop(), Some(2.0));
    assert_eq!(check.pop(), Some(1.0));
    assert_eq!(check.pop(), None);

    check.push(5.0);
    assert_eq!(check.apply(Op::Add), None);
    assert_eq!(check.items, vec![5.0]);
    check.pop();

    check.push(10.0);
    check.push(4.0);
    assert_eq!(check.apply(Op::Sub), Some(()));
    assert_eq!(check.pop(), Some(6.0));

    check.push(1.0);
    check.push(0.0);
    assert_eq!(check.apply(Op::Div), None);
    assert_eq!(check.items, vec![1.0, 0.0]);

    let mut stack = Stack::new();
    stack.push(2.0);
    stack.push(3.0);
    stack.apply(Op::Add).unwrap();
    stack.push(4.0);
    stack.apply(Op::Mul).unwrap();
    stack.push(1.0);
    stack.apply(Op::Sub).unwrap();

    while let Some(v) = stack.pop() {
        println!("{}", v); // 19
    }
}
```
