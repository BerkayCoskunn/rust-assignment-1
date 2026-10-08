# assignment 1 - my answers

hi, here are my 10 tasks for the assignment (5 from part 1 and 5 from part 2).

## part 1

**a)** 
error was E0382. the problem is that print_len takes ownership of the string and drops it. so i changed it to take a reference `&String` instead.

```rust
fn print_len(s: &String) { println!("{}", s.len()); }
fn main() {
    let s = String::from("hello");
    print_len(&s); // passed by ref
    println!("{s}");
}
```

**b)**
error E0502. we cant borrow v as mutable when first is borrowing it as immutable. since i32 is a simple number, i just removed the `&` so it copies the value of v[0] to `first`.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = v[0]; // just copy it
    v.push(4);
    println!("first = {first}");
}
```

**f)**
error E0505. shout function takes the name string and moves it. but we still need `name` for the `r` variable. i changed shout to borrow the string.

```rust
fn shout(s: &String) -> String { s.to_uppercase() }
fn main() {
    let name = String::from("ada");
    let r = &name;
    let loud = shout(&name); // borrow instead of move
    println!("{r} {loud}");
}
```

**g)**
error E0502. i cant push to `scores` while im inside a for loop that is reading `scores`. rust doesnt allow that. so i made a new vector to keep the numbers > 15, and added them to scores after the loop is done.

```rust
fn main() {
    let mut scores = vec![10, 20, 30];
    let mut to_add = Vec::new();
    
    for s in &scores {
        if *s > 15 {
            to_add.push(*s);
        }
    }
    
    // add them later
    for num in to_add {
        scores.push(num);
    }
    
    println!("{scores:?}");
}
```

**h)**
error E0499. you cant have two `&mut` at the same time for `c`. to fix this i just deleted `a` and `b` variables and called the bump function directly.

```rust
struct Counter { value: i32 }
impl Counter {
    fn bump(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
}
fn main() {
    let mut c = Counter { value: 0 };
    // just call them one by one
    println!("{} {}", c.bump(), c.bump());
}
```

---

## part 2

**1. first_word**
i used split by space. `next()` gives the first part. if it fails i just return the whole string.

```rust
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or(s)
}
```

**2. count_vowels**
i made a simple counter and checked each character with a for loop. i made it lowercase first so it catches 'A' and 'a'.

```rust
fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        let lower = c.to_ascii_lowercase();
        if lower == 'a' || lower == 'e' || lower == 'i' || lower == 'o' || lower == 'u' {
            count += 1;
        }
    }
    count
}
```

**3. reverse_words**
i put the words in a vector, reversed the vector, and joined them back with a space.

```rust
fn reverse_words(s: &str) -> String {
    let mut words: Vec<&str> = s.split_whitespace().collect();
    words.reverse();
    words.join(" ")
}
```

**4. capitalize**
i took the first char, made it uppercase, and then attached the rest of the string to it.

```rust
fn capitalize(s: &str) -> String {
    if s.len() == 0 {
        return String::new();
    }
    
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let rest = chars.as_str();
    
    first.to_uppercase().to_string() + rest
}
```

**5. is_palindrome**
i created a new string with only valid letters and numbers, then reversed it to see if it matches the original.

```rust
fn is_palindrome(s: &str) -> bool {
    let mut clean_string = String::new();
    
    for c in s.chars() {
        if c.is_alphanumeric() {
            clean_string.push(c.to_ascii_lowercase());
        }
    }
    
    let reversed: String = clean_string.chars().rev().collect();
    
    clean_string == reversed
}
```