# assignment 2 - my answers

hi, here are my first 10 mini tasks for the assignment.

## task 4 - vectors and slices

**1. double_in_place**
i used a mutable slice because the function changes the values in the original vector. i go through each number and multiply it by 2.

```rust
fn double_in_place(values: &mut [i32]) {
    for value in values {
        *value *= 2;
    }
}
```

**2. largest**
i used `Option<i32>` because the slice can be empty. if it is empty, `first()` returns `None`. otherwise i compare the numbers and keep the largest one.

```rust
fn largest(values: &[i32]) -> Option<i32> {
    let mut biggest = *values.first()?;

    for &value in values {
        if value > biggest {
            biggest = value;
        }
    }

    Some(biggest)
}
```

**3. task4_vectors**
i remove repeated numbers by keeping only the first time each number appears. then i double the numbers, calculate the running sum, and print the largest number.

```rust
fn remove_duplicates(values: &mut Vec<i32>) {
    let mut unique = Vec::new();

    for &value in values.iter() {
        if !unique.contains(&value) {
            unique.push(value);
        }
    }

    *values = unique;
}

fn running_sum(values: &[i32]) -> Vec<i32> {
    let mut sums = Vec::new();
    let mut total = 0;

    for &value in values {
        total += value;
        sums.push(total);
    }

    sums
}

fn task4_vectors() {
    let mut v = vec![3, 1, 3, 2, 1];
    remove_duplicates(&mut v);
    double_in_place(&mut v);

    println!("{:?}", v);
    println!("{:?}", running_sum(&v));
    println!("{:?}", largest(&v));
}
```

## task 5 - structs and methods

i used `Point` for the top-left position and `Rectangle` for the size. the y coordinate grows downwards, so the bottom edge is `top_left.y + height`.

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}
```

**4. area**
i multiply the width by the height.

```rust
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

**5. contains**
i check if the point is between the left, right, top, and bottom edges. the edges are included.

```rust
impl Rectangle {
    fn contains(&self, p: Point) -> bool {
        p.x >= self.top_left.x
            && p.x <= self.top_left.x + self.width
            && p.y >= self.top_left.y
            && p.y <= self.top_left.y + self.height
    }
}
```

**6. can_hold**
i compare only the sizes, because the task says to ignore the positions.

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
```

**7. scale**
i multiply both dimensions by the factor. `&mut self` lets me change the rectangle.

```rust
impl Rectangle {
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}
```

## task 6 - a library

**8. Book::new**
i use `String::from` to make the book own its title and author strings.

```rust
struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book {
    fn new(title: &str, author: &str, year: u16) -> Self {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
        }
    }
}
```

**9. Library::new**
i start with an empty vector of books.

```rust
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }
}
```

**10. add**
the function takes ownership of the book and puts it in the library's vector.

```rust
impl Library {
    fn add(&mut self, book: Book) {
        self.books.push(book);
    }
}
```
