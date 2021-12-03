# Neptune
Toy programming language with syntax inspired by C, Go etc.

## Progress
- [x] Write a lexer using Logos that can handle most of the grammar
- [ ] Write a recursive descent parser that can build an AST
- [ ] Represent the language as an AST
- [ ] Implement a type-checking pass, and constant reassignment pass
- [ ] Compile to C code

## Examples

### Hello world
```nim
# use := for declaration, this helps you identify if its a definition
# or assignment without paying too close attention
def message :: str := "hello, world"

fun main() => void {
    println(message)
}
```

### Summing two numbers
```nim
# Sum two numbers up
fun add(x :: int, y :: int) => int {
    return x + y
}

fun main() => void {
    add(2, 4)
}
```