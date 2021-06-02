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

    

    // Trait Objects

    // There are two ways of using traits to write polymorphic code in Rust: trait objects and generics. 

    // Rust doesn't permit variables of type Write:
    use std::io::Write;

    let mut buf: Vec<u8> = vec![];
    let writer: Write = buf; // error, `Write` does not have a constant size

    // A variable's size has to be known at compile time, and types that implement Write can be any size.

    // The reason is, unlike Java or other languages that have an equivalent to std::io::Write is that when used, those languages automatically make the variable using it a reference.

    // What we want in Rust is the same thing, But in Rust, references are explicit:
    let mut buf: Vec<u8> = vec![];
    let writer: &mut Write = &mut buf; // ok

    // A ref to a trait type, like writer, is called a trait object. Like any other ref, a trait object points to some value, it has a lifetime, and it can be either mut or shared.

    // What makes a trait object diff is that Rust usually doesn't know the type of the referent at compile time. So a trait object includes a little extra info about the referent's type. This is strictly for Rust's own use behind the scenes. When we call writer.write(data), Rust needs the type info to dynamically call the right method depending on the type of *writer. We can't query the type info directly, and Rust does not support downcasting from the trait object &mut Write back to a concrete type like Vec<u8>.



    // Trait Object Layout

    // In memory, a trait object is a fat pointer consisting of a pointer to the value, plus a pointer to a table representing that value's type. Each trait object therefore takes up two machine words (see page 378 for diagram).

    // It's called a virtual table, or vtable. In rust, the vtable is generated once, at compile time, and shared by all objects of the same type. Everything shown in dark grey (see table), including the vtable, is a private implementation detail of Rust. These aren't fields and data structures that we can access directly. Instead, the language automatically uses the vtable when we call a method of a trait object to determine which implementation to call.

    // In C++, the vtable pointer, or vptr, is stored as part of the struct. Rust uses fat pointers instead. The struct itself contains nothing but its fields. This way, a struct can implement dozens of traits without containing dozens of vptrs. Even types like i32, which aren't big enough to accommodate a vptr, can implement traits.

    // Rust automatically converts ordinary references into trait objects when needed. This is why we're able to pass &mut local_file to say_hello in this example:
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    // The type of &mut local_file is &mut File, and the type of the argument to say_hello is &mut Write. Since a File is a kind of writer, Rust allows this, automatically converting the plain reference to a trait object.

    // Likewise, Rust will happily convert a Box<File> to a Box<Write>, a value that owns a writer in the heap:
    let w: Box<Write> = Box::new(local_file);

    // Box<Write>, like &mut Write, is a fat pointer. It contains the address of the writer itself and the address of the vtable. The same goes for other pointer types, like Rc<Write>.

    // This kind of conversion is the only way to create a trait object. What the computer is actually doing here is very simple. At the point where the conversion happens, Rust knows the referent's true type (in this case, File), so it just adds the address of the appropriate vtable, turning the regular pointer into a fat pointer.



    // Generic Functions

    // At the start, we showed a say_hello() function that took a trait object as an argument. Let's rewrite that function as a generic function:
    fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
        out.write_all(b"hello world\n")?;
        out.flush()
    }

    // Only the type signature has changed:
    fn say_hello(out: &mut Write) // plain function

    fn say_hello<W: Write>(out: &mut W) // generic function

    // The phrase <W: Write> is what makes the function generic. This is a type parameter. It means that throughout the body of this function, W stands for some type that implements the Write trait. Type parameters are usually single uppercase letters, by convention.

    // Which type W stands for depends on how the generic function is used:
    say_hello(&mut local_file)?; // calls say_hello::<File>
    say_hello(&mut bytes)?; // calls say_hello<Vec<u8>>

    // When we pass &mut local_file to the generic say_hello() function, we're calling say_hello::<File>(). Rust generates machine code for this function that calls File::write_all() and File::flush(). When we pass &mut bytes, we're calling say_hello::<Vec<u8>>(). Rust generates separate machine code for this version of the function, calling the corresponding Vec<u8> methods. In both cases, Rust infers the type W from the type of the argument. We can always spell out the type parameters:
    say_hello::<File>(&mut local_file)?;

    // but it's seldom necessary, because Rust can usually deduce the type parameters by looking at the arguments. Here, the say_hello generic function expects a &mut W argument, and we're passing it a &mut File, so Rust infers that W = File.

    // If the generic function we're calling doesn't have any arguments that provide useful clues, we may have to spell it out:
    // calling a generic method collect<C>() that takes no arguments
    let v1 = (0 .. 1000).collect(); // error, can't infer type
    let v2 = (0 .. 1000).collect::<Vec<i32>>(); // ok

    // Sometimes we need multiple abilities from a type parameter. For example, if we want to print out the top 10 most common values in a vector, we'll need for those values to be printable:
    use std::fmt::Debug;

    fn top_ten<T: Debug>(values: &Vec<T>) { ... }

    // But this isn't good enough. How are we planning to determine which values are the most common? The usual way is to use the values as keys in a hash table. That means the values need to support the Hash and Eq operations. The bounds on T must include these as well as Debug. The syntax for this uses the + sign:
    fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) { ... }

    // Some types implement Debug, some implement Hash, some support Eq; and a few, like u32 and String, implement all three (see page 382 for diagram).

    // It's also possible for a type parameter to have no bounds at all, but we can't do much with a value if we haven't specified any bounds for it. We can move it, we can put it into a box or vector, that's about it.

    // Generic functions can have multiple type parameters:
    /// Run a query on a large, partitioned data set.
    /// See <http://research.google.com/archive/mapreduce.html>.
    fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
        data:&DataSet, map: M, reduce: R) -> Results
    { ... }

    // As this example shows, the bounds can get to be so long that they are hard on the eyes. Rust provides an alternative syntax using the keyword where:
    fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
        where M: Mapper + Serialize,
            R: Reducer + Serialize
    { ... }

    // The type parameters M and R are still declared up front, but the bounds are moved to separate lines. This kind of where clause is also allowed on generic structs, enums, type aliases, and methods. Anywhere bounds are permitted.

    // Of course, an alternative to where clauses is to keep it simple. Find a way to write the program without using generics quite so intensively.

    // "Receiving References as Parameters" (chapt 5) introduced the syntax for lifetime parameters. A generic function can have both lifetime parameters and type parameters. Lifetime parameters come first.
    /// Return a ref to the point in `candidates` that's
    /// closest to the `target` point.
    fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
        where P: MeasureDistance
    {
        ...
    }

    // This function takes two arguments, target and candidates. Both are references, and we give them distinct lifetimes 't and 'c (discussed in "Distinct Lifetime Parameters" in chapt 5). Furthermore, the function works with any type P that implements the MeasureDistance trait, so we might use it on Point2d values in one program and Point3d values in another.

    // Lifetimes never have any impact on machine code. Two calls to nearest() using the same type P, but diff lifetimes, will call the same compiled function. Only differing types cause Rust to compile multiple copies of a generic function.

    // Functions are not the only kind of generic code in Rust.
        // Already covered generic types in "Generic Structs" and "Generic Enums" in chapt 9 and 10 respectively.
        // An individual method can be generic, even if the type it's defined on is not generic:
        impl PancakeStack {
            fn push<T: Topping>(&mut self, goop: T) -> PancakeResult<()> {
                ...
            }
        }

        // Type aliases can be generic, too:
        type PancakeResult<T> = Result<T, PancakeError>;

        // Generic traits covered later in the chapt.

    

}
