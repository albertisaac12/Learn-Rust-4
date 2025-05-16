Debug mode is fast and unoptimized build , compiler includes additional metadata
Release mode takes longer to compile but optimizes runtime performance
final executable will be inside the target

`cargo check` checks for compiler violation // skips the creation of an executable

A compiler directive is when we add some meta data in the code which then directs the compiler to behave in a certain way.
We write the compiler directive just above the entity to which the compiler directive will be applied to.

Option<i32> and i32 are not the same

```txt

if let PATTERN = VALUE {
    // do something
}

the variable inside the pattern gets assigned with the value good if you want to use it for a single arm of match statement
```

What i understood from `let else` syntax:

Some variable x = Some other value { do stuff } else { do some other stuff }

Variable shadowing will occur in the same scope

constant values must be known at the compile time, need to explicitly specify the type.

precision: f32 6-9 digits , f64 15-17 digits

usize and isize : depends on the system architecture

string literals => value of these strings are known at the compile time. `&str`

`println!(" blah blah \" ");` `\` is a escape character.

`r" \m"` the `r` at the beginning tells rust to treat the entire string literal as a raw string.

method is a function that lives on that value.

`{:}` the `:` is a format specifier , the format specifier customizes the printed representation of the interpolated value

`vectors` are dynamic in size and are stored on the heap while `arrays` have fixed size and are stored in the stack

```plaintext
    A trait is like a contract that outlines certain rules, these rules are defined as methods
    when a type opts to honor a traits's requirements i.e the rules we say the type implements these traits
    Types can vary in their implementation but still implement the same trait

    In simple terms trait is an interface for a type.
```

`dbg!()` => file path + line + number of spaces in the line + the entire result in detail

The range type is not available in the top level of file and is nested in a `module`
