# Rust

## Introduction

Rust is a performant, safe, reliable, efficient and productive programming language. Almost*
all [AzatAI](https://azat.ai) projects
starting from July 2025 are built with Rust.
Installing rust:

- Ref: https://www.rust-lang.org/tools/install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
rustup self update
```

```bash
rustup update
```

## Basics

### Primitives

- Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Floats: `f32`, `f64`
- Booleans: `bool`
- Characters: `char` (Unicode scalar value)
- Unit type: `()`

Notes about char:

- Ref: https://doc.rust-lang.org/std/primitive.char.html
- Char is UTF-8 encoded.

### Variables

- Immutable by default: `let x = 5`
- Mutable: `let mut x = 5`
- Shadowing: `let x = 5; let x = 6;`
- Constants: `const PI: f64 = 3.14159265358979323846;`

### Compound Types

- Tuple: `let tup: (&str, i31) = ("Azat", 18);`
- Array: `let arr: [i32; 5] = [1, 2, 3, 4, 5];`
- Slicing: `&arr[1..4];`, `&arr[..4];`, `&arr[1..=3]`
- String: `String`, `&str`
- Struct: `struct User { name: String, age: i32 }`
- Enum: `enum Color { Red, Green, Blue }`

```rust
let tuple: (&str, i32) = ("Azat", 18);
println!("{} is {} years old", tuple.0, tuple.1)
```

```rust
let arr = [1, 2, 3, 4, 5];
println!("{:?}", arr);
```

```rust
let name: String = String::from("Azat");
let name = "Azat".to_string();
```

```rust
let name_str: &str = "Azat";
```

Note:

- Borrowing (&str) avoids heap allocation and is more performant when you just need to read values.
- use String if you need ownership, &str otherwise.

```rust
#[derive(Debug)]
    struct User {
        id: String,
        name: String,
        gender: Gender,
    }

    #[derive(Debug)]
    enum Gender {
        MALE,
        FEMALE,
    }

    let user = User {
        id: String::from("1"),
        name: "John".to_string(),
        gender: Gender::MALE,
    };

    println!("{:#?}", user)
```

### Flow Control

- `if/else`: `if condition { ... } else { ... }`
- `loop`: `loop { ... }` infinite loop, use `break` to exit.
- `while`: `while condition { ... }`
- `for`: `for item in collection { ... }`
- `match`: `match expression { pattern => value, ... }`, must cover all cases.

### Ownership

- Every value has a unique owner; when the owner goes out of scope, the value is dropped.
- There are two kinds of ownership:
    - Ownership of a value is passed to a function.
    - Ownership of a value is moved to a function.
- Move: `let s2 = s1`
- Clone: `let s2 = s1.clone()`

### Referencing and Lifetimes

- Immutable borrow: `&T`
- Mutable borrow: `&mut T`
- Borrowing rules: There can be only one mutable reference or multiple immutable references at a time
- Lifetimes: `'a`, assure that the borrowed value is valid for the lifetime `'a`.

```rust
#[derive(Debug)]
struct EnhancedQueryParser<'a> {
    query: &'a str,
    params: HashMap<&'a str, &'a str>
}

impl<'a> EnhancedQueryParser<'a> {
    fn from_string(query: &'a str) -> Self {
        let params: HashMap<&'a str, &'a str>  = query.split("&")
            .map(|kv| {
                let mut parts = kv.split("=");
                (
                    parts.next().unwrap(),
                    parts.next().unwrap()
                )
            })
            .collect();
        
        Self {
            query,
            params
        }
    }
}

fn main() {
    let qp2 = EnhancedQueryParser::from_string(&query);
    println!("params: {:#?}", qp2);
    let name = qp2.params.get("name").unwrap();
    println!("name: {}", name);
}
```

### Traits

Define a trait:

```rust
trait Summary {
    fn summarize(&self) -> String;
    fn default_behavior(&self) -> String {
        String::from("default behavior")
    }
}
```

Implement a trait for a type:

```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

Constraints:

```rust
fn notify<T: Summary>(item: &T) { ... } 

fn notify<T: Summary + Display>(item: &T) { ... }
```

Trait with where clause:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ ... }
```

