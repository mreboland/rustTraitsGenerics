use std::usize;

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

    

    // Which to Use

    // The choice of whether to use trait objects or generic code is subtle. Since both features are based on traits, they have a lot in common.

    // Trait objects are the right choice whenever we need a collection of values of mixed types, all together. It is technically possible to make generic salad:
    trait Vegetable {
        ...
    }

    struct Salad<V: Vegetable> {
        veggies: Vec<V>
    }

    // but this is a rather severe design. Each such salad consists entirely of a single type of vegetable.

    // How can we build a better salad? Since Vegetable values can be all diff sizes, we can't ask Rust for a Vec<Vegetable>:
    struct Salad {
        veggies: Vec<Vegetable> // error, `Vegetable` does not have a constant size
    }

    // Trait objects are the solution:
    struct Salad {
        veggies: Vec<Box<Vegetable>>
    }

    // Each Box<Vegetable> can own any type of vegetable, but the box itself has a constant size; two pointers, suitable for storing in a vector. Apart from the unfortunate mixed metaphor of having boxes in one's food, this is precisely what's called for, and it would work out just as well for shapes in a drawing app, monsters in a game, pluggable routing algorithms in a network router, and so on.

    // Another reason to use trait objects is to reduce the total amount of compiled code.

    // Generics have two important advantages over trait objects, with the result that in Rust, generics are the more common choice.

    // The first advantage is speed. Each time the Rust compiler generates machine code for a generic function, it knows which types it's working with, so it knows at that time which write method to call. There's no need for dynamic dispatch.

    // The generic min() function shown in the intro is just as fast as if we had written separate functions min_u8, min_i64, min_string, and so on. The compiler can inline it, like any other function, so in a release build, a call to min::<i32> is likely just two or three instructions. A call with constant arguments, like min(5, 3), will be even faster. Rust can evaluate it at compile time, so that there's no runtime cost at all.

    // Consider this generic function call:
    let mut sink = std::io::sink();
    say_hello(&mut sink)?;

    // std::io::sink() returns a writer of type Sink that quietly discards all bytes written to it.

    // When Rust generates machine code for this, it could emit code that call Sink::write_all, checks for errors, then calls Sink::flush. That's what the body of the generic function says to do.

    // Or, Rust could look at those methods and realize the following:
        // Sink::write_all() does nothing.
        // Sink::flush() does nothing.
        // Neither method ever returns an error.

    // In short, Rust has all the info it needs to optimize away this function entirely.

    // Compare that to the behaviour with trait objects. Rust never knows what type of value a trait object points to until run time. So even if we pass a Sink, the overhead of calling virtual methods and checking for errors still applies.

    // The second advantage of generics is that not every trait can support trait objects. Traits support several features, such as static methods, that work only with generics. They rule out trait objects entirely.


    
    // Defining and Implementing Traits

    // Defining a trait is simple. Give it a name and list the type signatures of the trait methods. If we're writing a game, we might have a trait like this:
    /// A trait for characters, items, and scenery -
    /// anything in the game world that's visible on screen
    trait Visible {
        /// Render this object on the given canvas.
        fn draw(&self, canvas: &mut Canvas);

        /// Return true if clicking at (x, y) should
        /// select this object
        fn hit_test(&self, s: i32, y: i32) -> bool;
    }

    // To implement a trait, use the syntax impl TraitName for type:
    impl Visible for Broom {
        fn draw(&self, canvas: &mut Canvas) {
            for y in self.y - self.height - 1 .. self.y {
                canvas.write_at(self.x, y, '|');
            }
            canvas.write_at(self.x, self.y, 'M');
        }

        fn hit_test(&self, s: i32, y: i32) -> bool {
            self.x == x
            && self.y - self.height - 1 <= y
            && y <= self.y
        }
    }

    // This impl contains an implementation for each method of the Visible trait, and nothing else. Everything defined in a trait impl must actually be a feature of the trait. If we wanted to add a helper method in support of Broom::draw(), we would have to define it in a separate impl block:
    impl Broom {
        /// Helper function used by Broom::draw() below
        fn broomstick_range(&self) -> Range<i32> {
            self.y - self.height - 1 .. self.y
        }
    }

    impl Visible for Broom {
        fn draw(&self, canvas: &mut Canvas) {
            for y in self.broomstick_range() {
                ...
            }
            ...
        }
        ...
    }



    // Default Methods

    // The Sink writer type we discussed earlier can be implemented in a few lines of code. First, we define the type:
    /// A Writer that ignores whatever data we write to it
    pub struct Sink;

    // Sink is an empty struct, since we don't need to store any data in it. Next, we provide an implementation of the Write trait for Sink:
    use std::io::{Write, Result};

    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            // Claim to have successfully written the whole buffer.
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    // So far, this is very much like the Visible trait. But we have also seen that the Write trait has a write_all method:
    out.write_all(b"hello world\n")?;

    // Why does Rust let us impl Write for Sink without defining this method? The answer is that the standard library's definition of the Write trait contains a default implementation for write_all:
    trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            let mut bytes_written = 0;
            while bytes_written < buf.len() {
                bytes_written += self.write(&buf[butes_written..])?;
            }
            Ok(())
        }

        ...
    }

    // The write and flush methods are the basic methods that every writer must implement. A writer may also implement write_all, but if not, the default implementation shown above will be used.

    // Our own traits can include default implementations using the same syntax. More in chapt 5.



    // Traits and Other People's Types

    // Rust lets us implement any trait on any type, as long as either the trait or the type is introduced in the current crate.

    // This means that any time we want to add a method to any type, we can use a trait to do it:
    trait IsEmoji {
        fn is_emoji(&self) -> bool;
    }

    /// Implement IsEmoji for the built-in character type
    impl IsEmoji for char {
        fn is_emoji(&self) -> bool {
            ...
        }
    }

    assert_eq!('$'.is_emoji(), false);

    // Like any other trait method, this new is_emoji method is only visible when IsEmoji is in scope.

    // The sole purpose of this particular trait is to add a method to an existing type, char. This is called an extension trait. We can also add this trait to types as well, by writing impl IsEmoji for str { ... } and so forth.

    // We can even use a generic impl block to add an extension trait to a whole family of types at once. The following extension trait adds a method to allRust writers:
    use std::io::{self, Write};

    /// Trait for values to which you can send HTML.
    trait WriteHtml {
        fn write_html(&mut self, &HtmlFocument) -> io::Result<()>;
    }

    /// We can write HTML to any std::io writer.
    impl<W: Write> WriteHtml for W {
        fn write_html(&mut self, &HtmlFocument) -> io::Result<()> {
            ...
        }
    }

    // The line impl<W: Write> WriteHtml for W means "for every type W that implements Write, here's an implementation of WriteHtml for W".

    // The serde library offers a nice example of how useful it can be to implement user-defined traits on standard types. serde is a serialization library. That is, we can use it to write Rust data structures to disk and reload them later. The library defines a trait, Serialize, that's implemented for every data type the library supports. So in the serde source code, there is code implementing Serialize for bool, i8, i16, i32, array and tuple types, and so on, through all the standard data structures like Vec and HashMap.

    // The upshot of all this is that serde add a .serialize() method to all these types. It can be used like so:
    use serde::Serialize;
    use serde_json;

    pub fn save_configuration(config: &HashMap<String, String) -> std::io::Result<()> {
        /// Create a JSON serializer to write the data to a file.
        let writer = FIle::create(config_filename())?;
        let mut serializer = serde_json::Serializer::new(writer);

        // The serde `.serialize()` method does the rest.
        config.serialize(&mut serializer)?;

        Ok(())
    }

    // We said earlier that when we implement a trait, either the trait or the type must be new in the current crate. This is called the coherence rule. It helps Rust ensure that trait implementations are unique. Our code can't impl Write for u8, because both Write and u8 are defined in the standard library. If Rust let crates do that, there could be multiple implementations of Write for u8, in different crates, and Rust would have no reasonable way to decide which implementation to use for a given method call.



    // Self in Traits

    // A trait can use the keyword Self as a type. The standard Clone trait, for example, looks like this (slightly simplified):
    pub trait Clone {
        fn clone(&self) -> Self;
        ...
    }

    // Using Self as the return type here means that the type of x.clone() is the same as the type of x, whatever that might be. If x is a String, then the type of x.clone() is String, not Clone, or any other cloneable type.

    // Likewise, if we define this trait:
    pub trait Spliceable {
        fn splice(&self, other:&Self) -> Self;
    }

    // with two implementations:
    impl Spliceable for CherryTree {
        fn splice(&self, other: &Self) -> Self {
            ...
        }
    }

    impl Spliceable for Mammoth {
        fn splice(&self, other: &Self) -> Self {
            ...
        }
    }

    // then inside the first impl, Self is simply an alias for CherryTree, and in the second, it's an alias for Mammoth. This means that we can splice together two cherry trees or two mammoths, not that we can create a mammoth-cherry hybrid. The type of self and the type of other must match.

    // A trait that uses the Self type is incompatible with trait objects:
    // error: the trait `Spliceable` cannot be made into an object
    fn splice_anything(left: &Spliceable, right: &Spliceable) {
        let combo = left.splice(right);
        ...
    }

    // The reason is something we'll see again and again as we dig into the advanced features of traits. Rust rejects this code because it has no way to type-check the call left.splice(right). The whole point of trait objects is that the type isn't know until runtime. Rust has no way to know at compile time if left and right will be the same type, as required.

    // Trait objects are really intended for the simplest kinds of traits, the kinds that could be implemented using interfaces in Java or abstract base classes in C++. The more advanced features of traits are useful, but they can't coexist with trait objects because with trait objects, we lose the type info Rust needs to type-check our program.

    // Now, had we wanted genetically improbably splicing, we could have designed a trait-object-friendly trait:
    pub trait MegaSpliceable {
        fn splice(&self, other: &MegaSpliceable) -> Box<MegaSpliceable>;
    }

    // This trait is compatible with trait objects. There's no problem type-checking calls to this .splice() method because the type of the argument other is not required to match the type of self, as long as both types are MegaSpliceable.



    // Subtraits

    // We can declare that a trait is an extension of another trait:
    /// Someone in the game world, either the player or some other
    /// pixie, gargoyle, squirrel, ogre, etc.
    trait Creature: Visible {
        fn position(&self) -> (i32, i32);
        fn facing(&self) -> Direction;
        ...
    }

    // The phrase trait Creature: Visible means that all creatures are visible. Every type that implements Creature must also implement the Visible trait:
    impl Visible for Broom {
        ...
    }

    impl Creature for Broom {
        ...
    }

    // We can implement the two trait in either order, but it's an error to implement Creature for a type without also implementing Visible.

    // Subtraits are a way to describe a trait that extends an existing trait with a few more methods. In this example, all our code that works with Creatures can also use the methods from Visible trait.



    // Static Methods

    // In most object-oriented languages, interfaces can't include static methods or constructors. However, Rust traits can include static methods and constructors, here is how:
    trait StringSet {
        /// Return a new empty set
        fn new() -> Self;

        /// Return a set that contains all the string in `strings`.
        fn from_slice(strings: &[&str]) -> Self;

        /// Find out if this set contains a particular `value`.
        fn contains(&self, string: &str) -> bool;

        /// Add a string to this set.
        fn add(&mut self, string: &str);
    }

    // Every type that implements the StringSet trait must implement these four associated functions. The first two, new() and from_slice(), don't take a self argument. They serve as constructors.

    // In nongeneric code, these functions can be called using :: syntax, just like any other static method:
    // Create sets of two hypothetical types that impl StringSet:
    let set1 = SortedStringSet::new();
    let set2 = HashedStringSet::new();

    // In generic code, it's the same, except the type is often a type variable, as in the call to S::new() shown here:
    /// Return the set of words in`document` that aren't in `wordlist`.
    fn unknown_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
        let mut unknowns = S::new();
        for word in document {
            if !wordlist.contains(word) {
                unknowns.add(word);
            }
        }
        unknowns
    }

    // Traits objects don't support static methods. If we want to use &StringSet trait objects, we must change the trait, adding the bound where Self: Sized to each static method:
    trait StringSet {
        fn new() -> Self
            where Self: Sized;

        fn from_slice(strings: &[&str]) -> Self
            where Self: Sized;

        fn contains(&self, string: &str) -> bool;

        fn add(&mut self, string: &str);
    }

    // This bound tells Rust that trait objects are excused from supporting this method. StringSet trait objects are then allowed. They still don't support the two static methods, but we can create them and use them to call .contains() and .add(). The same trick works for any other method that is incompatible with trait objects.



    // Fully Qualified Method Calls

    // A method is just a special kind of function. These two calls are equivalent:
    "hello".to_string()
    str::to_string("hello")

    // The second form looks exactly like a static method call. This works even though the to_string method takes a self argument. Simply pass self as the function's first argument.

    // Since to_string is a method of the standard ToString trait, there are two more forms we can use:

    ToString::to_string("hello")
    <str as ToString>::to_string("hello")

    // ALl four of these method calls do exactly the same thing. Most often, we'll just write value.method(). The other forms are qualified method calls. They specify the type or trait that a method is associated with. The last form, with the angle brackets, specifies both, a fully qualified method call.

    // When we write "hello".to_string(), using the . operator, we don't say exactly which to_string method we're calling. Rust has a method lookup algorithm that figures this out, depending on the types, deref coercions, and so on. With fully qualified calls, we can say exactly which method we mean, and that can help in a few odd cases:
        // 1. When two methods have the same name. The classic hokey example is the Outlaw with two .draw() methods from two diff traits. One for drawing it on the screen and one for interacting with the law:
        outlaw.draw(); // error, draw on screen or draw pistol?
        Visible::draw(&outlaw); // ok, draw on screen
        HasPistol::draw(&outlaw); // ok, corral
        // Normally, we're better off just renaming one of the methods, but sometimes we can't.

        // 2. When the type of the self argument can't be inferred:
        let zero = 0; // type unspecified, could be `i8`, `u8`, ...
        zer.abs(); // error, method `abs` not found
        i64::abs(zero); // ok

        // 3. When using the function itself as a function value:
        let words: Vec<String> =
            line.split_whitespace() // iterator produces &str values
                .map(<str as ToString>::to_string) // ok
                .collect();
        // Here the fully qualified <str as ToString>::to_string is just a way to name the specific function we want to pass to .map().
        
        // 4. When calling trait methods in macros (chapt 20).
    
    // Fully qualified syntax also works for static methods. In the previous section, we wrote S::new() to create a new set in a generic function. We could also have written StringSet::new() or <S as StringSet>::new().



    // Traits That Define Relationships Between Types

    // A trait is a set of methods that types can implement. Traits can also be used in situations where there are multiple types that have to work together. They can describe relationship between types.
        // 1. The std::iter::Iterator trait relates each iterator type with the type of value it produces.
        // 2. The std::ops::Mul trait relates types that can be multiplied. In the expression a * b, the values a and b can be either the same type or diff types.
        // 3. The rand crate includes both a trait for random number generators (rand::Rng) and a trait for types that can be randomly generated (rand::Rand). The traits themselves define exactly how these types work together.

    

    // Associated Types (or How iterators Work)

    // Rust has a standard Iterator trait, defined like so:
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
        ...
    }

    // The first feature of this trait, type item;, is an associated type. Each type that implements Iterator must specify what type of item it produces.

    // The second feature, the next() method uses the associated type in its return value. next() returns an Option<Self::Item>. Either Some(item), the next value in the sequence, or None, when there are no more values to visit. The type is written as Self::Item, not just plain Item, because Item is a feature of each type of iterator, not a standalone type. As always, self and the Self type show up explicitly in the code everywhere their fields, methods, and so on are used.

    // Here's what it looks like to implement Iterator for a type:
    // (code from the std::env standard lib module)
    impl Iterator for Args {
        type Item = String;

        fn next(&mut self) -> Option<String> {
            ...
        }
        ...
    }

    // std::env::Args is the type of iterator returned by the standard lib function std::env::args() that we used in chapt 2 to access command-line arguments. It produces String values, so the impl declares type item = String;.

    // Generic code can use associated types:
    /// Loop over an iterator, storing the values in a new vector.
    fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
        let mut results = Vec::new();
        for value in iter {
            results.push(value);
        }
        results
    }

    // Inside the body of the above function, Rust infers the type of value for us. But we must spell out the return type of collect_into_vector, and the Item associated type is the only way to do that. (Vec<I> would be wrong, we would be claiming to return a vector of iterators!)

    // The preceding example is not code that we would write out ourself. After reading chapt 15, we'll know that iterators already have a standard method that does thi: iter.collect(). Another example:
    /// Print out all the values produced by an iterator
    fn dump<I>(iter: I)
        where I: Iterator
    {
        for (index, value) in iter.enumerate() {
            println!("{}: {:?}", index, value); // error
        }
    }

    // The above almost works, there is just one problem. value might not be a printable type.
    // error: the trait bound <I as std::iter::... is not satisfied...

    // The error message is slightly obfuscated by Rust’s use of the syntax <I as std::iter::Iterator>::Item, which is a long, maximally explicit way of saying I::Item. This is valid Rust syntax, but we'll rarely actually need to write a type out that way.

    // The gist of the error msg is that to make this generic function compile, we must ensure that I::Item implements the Debug trait, the trait for formatting values with {:?}. We can do this by placing a bound on I::Item:
    use std::fmt::Debug;

    fn dump<I>(iter: I)
        where I: Iterator, I::Item: Debug
    {
        ...
    }

    // Or, we could write, "I must be an iterator over String values":
    fn dump<I>(iter: I)
        where I: Iterator<Item=String>
    {
        ...
    }

    // Iterator<Item=String> is itself a trait. If we think of Iterator as the set of all iterator types, then Iterator<Item=String> is a subset of Iterator. The set of iterator types that produce Strings. This syntax can be used anywhere the name of a trait can be used, including trait object types:
    fn dump(iter: &mut Iterator<Item=String>) {
        for (index, s) in iter.enumerate() {
            println!("{}: {:?}", index, s);
        }
    }

    // Traits with associated types, like Iterator, are compatible with trait methods, but only if all the associated types are spelled out, as shown. Otherwise, the type of s could be anything, and again, Rust would have no way to type-check this code.

    // Iterators are by far the most prominent use of associated types. But associated types are generally useful whenever a trait needs to cover more than just methods.
        // In a thread pool library, a Task trait, representing a unit of work, could have an associated Output type.
        // A Pattern trait, representing a way of searching a string, could have an associated Match type, representing all the info gathered by matching the pattern to the string.
        trait Patter {
            type Match;

            fn search(&self, string: &str) -> Option<Self::Match>;
        }

        /// We can search a string for a particular character.
        impl Pattern for char {
            /// A "match" is just the location where the character was found
            type Match = usize;

            fn search(&self, string: &str) -> Option<usize> {
                ...
            }
        }

        // If we're familiar with regular expressions, it's easy to see how impl Pattern for RegExp would have a more elaborate Match type, probably a struct that would include the start and length of the match, the locations hwere parenthesized groups matched, and so on.
        
        // A library for working with relational databases might have a DatabaseConnection trait with associated types representing transactions, cursors, prepared statements, and so on.
    
    // Associated types are perfect for cases where each implementation has one specific related type. Each type of Task produces a particular type of Output. Each type of Pattern looks for a particular type of Match.



    // Generic Traits (or How Operator Overloading Works)

    // Multiplication in Rust uses this trait:
    /// std::ops::Mul, the trait for types that support `*`.
    pub trait Mul<RHS> {
        /// The resulting type after applying the `*` operator
        type Output;

        /// The methods for the `*` operator
        fn mul(self, rhs: RHS) -> Self::Output;
    }

    // Mul is a generic trait. The type parameter, RHS, is short for right hand side.

    // The type parameter here means the same thing that it means on a struct or function. Mul is a generic trait, and its instances Mul<f64>, Mul<String>, Mul<Size>, etc. are all diff traits, just as min::<i32> and min::<String> are diff functions and Vec<i32> and Vec<String> are diff types.

    // A single type, say, WindowSize, can implement both Mul<f64> and Mul<i32>, and many more. We would then be able to multiply a WindowSize by many other types. Each implementation would have its own associated Output type.

    // The trait shown above is missing on minor detail. The real Mul trait looks like this:
    pub trait Mul<RHS=Self> {
        ...
    }

    // The syntax RHS=Self means that RHS defaults to Self. If we write impl Mul for Complex, without specifying Mul's type parameter, it means impl Mul<Complex> for Complex. In a bound, if we write where T: Mul, it means where T: Mul<T>.

    // In Rust, the expression lhs * rhs is shorthand for Mul::mul(lhs, rhs). So overloading the * operator in Rust is as simple as implementing the Mul trait.



    // Buddy Traits (or How rand::random() Works)

    // There's one more way to use traits to express relationships between types. This way is perhaps the simplest of the bunch, since we don't have to learn any new language features to understand it. What we'll call buddy traits, are simply traits that are designed to work together.

    // There's a good example inside the rand crate, a popular crate for generating random numbers. The main feature of rand is the random() function, which returns a random value:
    use rand::random;
    let x = random();

    // If Rust can't infer the type of the random value, which is often the case, we must specify it:
    let x = random::<f64>(); // a number, 0.0 <= x < 1.0
    let b = random::<bool>(); // true or false

    // For many programs, this one generic function is all we need. But the rand crate also offers several diff, but interoperable, random number generators. All the random number generators in the library implement a common trait:
    /// A random number generator.
    pub trait Rng {
        fn next_u32(&mut self) -> u32;
        ...
    }

    // An Rng is simply a value that can spit out integers on demand. The rand library provides a few diff implementations, including XorShiftRng (a fast pseudorandom number generator) and OsRng (much slower, but truly unpredictable, for use in cryptography).

    // The buddy trait is called Rand:
    /// A type that can be randomly generated using an `Rng`.
    pub trait Rand: Sized {
        fn rand<R: Rng>(rng: &mut R) -> Self;
    }

    // Types like f64 and bool implement this trait. Pass any random number generator to their ::rand() method, and it returns a random value:
    let x = f64::rand(rng);
    let b = bool::rand(rng);

    // In fact, random() is nothing but a thin wrapper that passes a globally allocated Rng to this rand method. One way to implement it is like this:
    pub fn random<T: Rand>() -> T {
        T::rand(&mut global_rng())
    }

    // When we see traits that use other traits as bounds, the way Rand::rand() uses Rng, we know that those two traits are mix-and-match. Any Rng can generate values of every Rand type. Since the methods involved are generic, Rust generates optimized machine code for each combination of Rng and Rang that our program actually uses.

    // The two traits also serve to separate concerns. Whether we're implementing Rand for our Monster type or implementing a spectacularly fast, but not-so-random Rng, we don't have to do anything special for those two pieces of code to be able to work together (see fig on page 411).
    
    // The standard lib's support for computing hash codes provides another example of buddy traits. Types that implement Has are hashable, so they can be used as hash table keys. Types that implement Hasher are hashing algorithms. The two are linked in the same way as Rand and Rng. Hash has a generic method Hash::hash() that accepts any type of Hasher as an argument.



    // Reverse-Engineering Bounds

    // Writing generic code can be a real slog when there's no single trait that does everything we need. Suppose we have written this nongeneric function to do some computation:
    fn dot(v1: &[i64], v2: &[i64]) -> i64 {
        let mut total = 0;
        for i in 0 .. v1.len() {
            total = total + v1[i] * v2[i];
        }
        total
    }

    // Now we want to use the same code with floating-point values. We might try something like this:
    fn dot<N>(v1: &[N], v2: &[N]) -> N {
        let mut total: N = 0;
        for i in 0 .. v1.len() {
            total = total + v1[i] * v2[i];
        }
        total
    }

    // No such luck. Rust complains about the use of + and * and the type of 0. We can require N to be a type that supports + and * using the Add and Mul traits. Our use of 0 needs to change, though, because 0 is always an integer in Rust. The corresponding floating-point value is 0.0. Fortunately, there is a standard Default trait for types that have default values. For numeric types, the default is always 0.
    use std::ops::{Add, Mul};

    fn dot<N: Add + Mul + Default>(v1: &[N], v2: &[N]) -> N {
        let mut total = N::default();
        for i in 0 .. v1.len() {
            total = total + v1[i] * v2[i];
        }
        total
    }

    // The above is closer, but still does not work:
    // error: mismatched types...
    // traits_generic_dot_2.rs...

    // Our new code assumes that multiplying two values of type N produces another value of type N. This isn't necessarily the case. We can overload the multiplication operator to return whatever type we want. We need to somehow tell Rust that this generic function only works with types that have the normal flavour of multiplication, where multiplying N * N returns an N. We do this by replacing Mul with Mul<Output=N>, and the same for Add:
    fn dot<N: Add<Output=N> + Mul<Output=N> + Default>(v1: &[N], v2: &[N]) -> N
    {
        ...
    }

    // At this point, the bounds are starting to pile up, making the code hard to read. Let's move the bounds into a where clause:
    fn dot<N>(v1: &[N], v2: &[N]) -> N
        where N: Add<Output=N> + Mul<Output=N> + Default
    {
        ...
    }

    // Rust still complains about the above code:
    // Error: cannot move out of type `[N]`, a non-copy array..
    // traits_generic_dot_3...

    // It illegal to move the value of v1[i] out of the slice. But numbesr are copyable, so what's the issue?

    // The answer is that Rust doesn't know v1[i] is a number. In fact, it isn't. The type N can be any type that satisfies the bounds we've given it. If we also want N to be a copyable type, we must say so:
    where N: Add<Output=N> + Mul<Output=N> + Default + Copy

    // With this, the code compiles and runs. The final code looks like this:
    use std::ops::{Add, Mul};

    fn dot<N>(v1: &[N], v2: &[N]) -> N
        where N: Add<Output=N> + Mul<Output=N> + Default + Copy
    {
        let mut total = N::default();
        for i in 0 .. v1.len() {
            total = total + v1[i] * v2[i];
        }
        total
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&[1,2,3,4], &[1,1,1,1]), 10);
        assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
    }

    // This occasionally happens in Rust. There is a period of intense arguing with the compiler, at the end of which the code looks rather nice, as if it had been a breeze to write, and runs beautifully.

    // What we've been doing here is reverse-engineering the bounds on N using the compiler to guide and check our work. The reason it was a bit of a pain is that there wasn't a single Number trait in the std lib that included all the operators and methods we wanted to use. As it happens, there's a popular open source crate called num that defines such a trait. We could have added num to our Cargo.toml and written:
    use num::Num;

    fn dot<N: Num + Copy>(v1: &[N], v2: &[N]) -> N {
        let mut total = N::zero();
        for i in 0 .. v1.len() {
            total = total + v1[i] * v2[i];
        }
        total
    }

    // Just as in OOP, the right interface makes everything nice. In generic programming, the right trait makes everything nice.

    // Once advantage of Rust's approach (vs C++) is forward compatibility of generic code. We can change the implementation of a public generic function or method, and if we didn't change the signature, we haven't broken any of its users.

    // Another advantage of bounds is that when we do get a compiler error, it as least tells you where the trouble is. C++ is much more convoluted, is it a template, or the template's caller?

    // Perhaps the most important advantage of writing out the bounds explicitly is simply that they are there, in the code and in the documentation. We can look at the signature of a generic function in Rust and see exactly what kind of arguments it aceepts.





    

}
