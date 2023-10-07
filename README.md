# Compiler hmmmm

The idea of this is to build a standalone compiler that compiles a typed python / rust-esque language to machine code. (assembly
Learning project from COMP3710 special topics course on compilers @ ANU.

Ideas:

- syntactic whitespace
- no need for `var` or `let`
- easy javascript like definition of closures
  - this will create some cursed frame pointer shenanigans later

## Language Vibe (opposite of a rigorous spec):

```

def main():
    x = 123 # x is inferred as typeof int
    y: string


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
- Function: `(arg1, arg2, arg3): return_type` Stored as a pointer to some code that defines the function.
