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
