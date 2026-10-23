// Assignment 2 - first 10 mini tasks
// Vectors, structs, owning data and the VM value stack

// Task 4: Vectors and slices

fn double_in_place(values: &mut [i32]) {
    for value in values {
        *value *= 2;
    }
}

fn largest(values: &[i32]) -> Option<i32> {
    let mut biggest = *values.first()?;

    for &value in values {
        if value > biggest {
            biggest = value;
        }
    }

    Some(biggest)
}

// Keep the first occurrence of each number.
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

    println!("Task 4: {:?}, {:?}, {:?}", v, running_sum(&v), largest(&v));
}

// Task 5: Structs and methods

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

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // The edges count as being inside the rectangle.
    // The y coordinate grows downwards, so the bottom is top_left.y + height.
    fn contains(&self, p: Point) -> bool {
        p.x >= self.top_left.x
            && p.x <= self.top_left.x + self.width
            && p.y >= self.top_left.y
            && p.y <= self.top_left.y + self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// Task 6: A library (first three mini tasks)

#[derive(Debug)]
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

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    // This takes ownership of the book and stores it in the library.
    fn add(&mut self, book: Book) {
        self.books.push(book);
    }
}

fn main() {
    task4_vectors();

    let mut rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 5.0,
    };
    let smaller = Rectangle {
        top_left: Point { x: 2.0, y: 1.0 },
        width: 4.0,
        height: 3.0,
    };
    println!("Area: {}", rectangle.area());
    println!("Contains point: {}", rectangle.contains(Point { x: 10.0, y: 5.0 }));
    println!("Can hold smaller rectangle: {}", rectangle.can_hold(&smaller));
    rectangle.scale(2.0);
    println!("Scaled rectangle: {:?}", rectangle);

    let mut library = Library::new();
    library.add(Book::new("The Hobbit", "J.R.R. Tolkien", 1937));
    println!("Library: {:?}", library);
}

