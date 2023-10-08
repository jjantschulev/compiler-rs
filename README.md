# Compiler hmmmm

_The goal of this document is to be as out of date as possible. Do not rely on it, read the source code_

The idea of this is to build a standalone compiler that compiles a typed ~~python / rust-esque~~ \*typescripty language to machine code. (assembly
Learning project from COMP3710 special topics course on compilers @ ANU.

Ideas:

- ~~syntactic whitespace~~
  - this is cursed, we are using semicolons, and curly braces
- ~~no need for `var` or `let`~~
  - um yes, there is need. will use `let`
- easy javascript like definition of closures ✅
  - this will create some cursed frame pointer, and heap allocated captured variable shenanigans later
- idea is to look like typescript, but behave like `c`

## Language Vibe (opposite of a rigorous spec):

_again, idea here that is is suuuper out of date_

```

type T = {
  field: int,
  fun: (bool, &char) => int,
};

let x: T = {
  field: 1,
  fun: (a: bool, b: &char) => {
    return 1;
  },
};

let main = () => {
  let y: int = x.fun(true, &'a');
  return y;
};

```

## Types

### Primitives:

All primitives are stored as words. Types are just for compile type type checking. You can freely cast between all primitives

- `int`: machine sized word (signed)
- `uint`: machine sized word (unsigned)
- `float`: machine sized floating point
- `char`: 1 byte. (still stored as a word)
- `boolean`: true | false (also stored as a word)
- `ptr`: pointer to an address. pointers are typed. void pointers are allowed!!! I want unsafety because its powerful
- `void`: no type.
- `string`: an array of bytes. length is a prefixed word. equivalent to `char[]`

### Composites:

- Array: `T[n]`: fixed length array with n spaces of size `sizeof(T)` elements.
- List: `T[]`: variable length array with n spaces of size `sizeof(T)` elements. The length is stored as a prefixed word in the memory layout. This is always stored as a reference. Never inline
- Struct: `{ field: type }` a collection of named fields
- Function: `(arg1, arg2, arg3) => return_type` Stored as a pointer to some code that defines the function.

# Exceptions

Exceptions are thrown with the `yeet` keyword. That is all I care about rn. Maybe you can catch them with the `sike, you thought` keyword. Maybe not. Who knows.

# License

not open source, reference me wen u reuse the source code.

enterprise cost: $2.99 per line of code compiled.
