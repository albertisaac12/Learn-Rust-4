Debug mode is fast and unoptimized build , compiler includes additional metadata
Release mode takes longer to compile but optimizes runtime performance
final executable will be inside the target

`cargo check` checks for compiler violation // skips the creation of an executable

A compiler directive is when we add some meta data in the code which then directs the compiler to behave in a certain way.

Option<i32> and i32 are not the same

```txt

if let PATTERN = VALUE {
    // do something
}

the variable inside the pattern gets assigned with the value good if you want to use it for a single arm of match statement
```
