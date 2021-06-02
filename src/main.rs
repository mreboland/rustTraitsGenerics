fn main() {
    println!("Hello, world!");



    // Traits and Generics

    // One of the great discoveries in programming is that it's possible to write code that operates on values of many different types, even types that haven't been invented yet. For example:
        // 1. Vec<T> is generic: we can create a vector of any type of value, including types defined in our program that the authors of Vec never anticipated.
        // 2. Many things have .write() methods, including Files and TcpStreams. Our code can take a writer by ref, any writer, and send data to it. Our code doesn't have to care what type of writer it is. Later, if someone adds a new type of writer, our code will already support it.

    // This capability isn't new, and is called polymorphism. Rust supports polymorphism with two related features: traits and generics. These concepts will be familiar to many programmers, but Rust takes a fresh approach inspired by Haskell's typeclasses.

    // Traits are Rust's take on interfaces or abstract base classes. At first, they look just like interfaces in Java or C#. The trait for writing bytes is called std::io::Write, and its definition in the standard lib starts out like so:
    trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }
        ...
    }

    // This traits offers several methods that we've only shown 3 of.

    // The standard types File and TcpStream both implement std::io::Write. So does Vec<u8>. All three types provide methods named .write(), .flush(), and so on. Code that uses a writer without caring about its type looks like this:
    use std::io::Write;

    fn say_hello(out: &mut Write) -> std::io::Result<()> {
        out.write_all(b"hello world\n")?;
        out.flush()
    }

    // The type of out is &mut Write, meaning "a mutable ref to any value that implements the Write trait".
    use std::fs::File;

    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?; // works

    let mut bytes = vec![];
    say_hello(&mut bytes)?; // also works
    assert_eq!(bytes, b"hello world\n");

    // Generics are the other flavour of polymorphism in Rust. Like a C++ template, a generic function or type can be used with values of many different types.
    /// Given two values, pick whichever one is less.
    fn min<T: Ord>(value1: T, value2: T) -> T {
        if value1 <= value2 {
            value1
        } else {
            value2
        }
    }

    // The <T: Ord> in this function means that min can be used with arguments of any type T that implements the Ord trait. That is, any ordered type. The compiler generates custom machine for each type T that we actually use.

    // Generics and traits are closely related. Rust makes us declare the T: Ord requirement (called a bound) up front, before using the <= operator to compare two values of type T. We'll also cover how &mut Write and <T: Write> are similar, how they're diff, and how to choose between these two ways of using traits.



    // Using Traits

    // A trait is a feature that any given type may or may not support. Most often, a trait represents a capability, something a type can do.
        // A value that implements std::io::Write can write out bytes
        // A value that implements std::iter::Iterator can produce sequence of value
        // A value that implements std::clone::Clone can make clones of itself in memory
        // A value that implements std::fmt::Debug can be printed using println!() with the {:?} format specifier.

    // These traits are all part of Rust's standard library, and many standard types implement them.
        // std::fs::File implements the Write trait. It writes bytes to a local file. std::net::TcpStream writes to a network connection. Vec<u8> also implements Write. Each .write() call on a vector of bytes appends some data to the end.
        // Range<i32> (the type of 0..10) implements the Iterator trait, as do some iterator types associated with slices, hash tables, and so on.
        // Most standard library types implement Clone. The exceptions are mainly types like TcpStream that represent more than just data in memory.
        // Likewise, most standard library types support Debug.

    // There is one unusual rule about trait methods. The trait itself must be in scope, otherwise, all its methods are hidden.
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello")?; // error, no method named `write_all`

    // In this case, the compiler prints a friendly error message that suggests adding use std::io::Write, and indeed that fixes the problem:
    use std::io::Write;

    let mut...
    buf.write...

    // Rust has this rule because, as we'll see later in the chapt, we can use traits to add new methods to any type, even standard lib types like u32 and str. Third-party crates can do the same thing.

    // The reason Clone and Iterator methods work without any special imports is that they're always in scope by default. They're part of the standard prelude, names that Rust automatically imports into every module. Mor on preludes in chapt 13.

    

    

}
