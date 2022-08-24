# Rust

Rust is a language that provides a trifecta: Security, Concurrency and Speed.

Syntaxis = Mix of Python, C and C++. Mix of high/low level languages.

It is a strongly typed language.

- Cargo = Package Manager

1. Cargo Build
2. Cargo Run
3. Cargo Check (Compiles without running the program)

- Variables

  They are immuatable by default.
  
  ```rust
  let variable_name = 2;  
  ```
  
  They can be mutated adding "mut".
  
    ```rust
  let variable_name = 2;
  variable_name = 10;
  ```
  
 - Functions
 
    They are declared with fn, the name of the function goes in snake_case and parameteres are defined with their corresponding type.
    Also the return type of the function is declared with "->".
    
    ```rust
    fn function_name(number: i32) -> i32 {
      number
    }

    fn greeting() {
      printLn!("Hello World!")
    }
    ```
  
  - Module System


    Use statement = Brings an item from some scope (similar like an import)
    Crates = Compilation unit in rust (kinda like packages)
  
  
    - src/lib.rs
    
    ```rust
    pub fn greet() {
      printLn!("Hi!")
    }
    ```
    
    - src/main.rs
    
    ```rust
    
    use hello::greet;
    
    fn main() {
      greet()
    }
    ```
    
 - Scalar Types

    - Integers:

      - Unsigned:

          1. u8
          2. u16
          3. u32
          4. u64
          5. u128
          6. usize

      - Signed:

        1. i8
        2. i16
        3. i32
        4. i64
        5. i128
        6. isize
   
   
