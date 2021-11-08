# FunkshunL
FunkshunL is an esoteric programming language with no flow control and broken function call semantics.

# Design

FunkshunL is a tape language (like brainfuck) with no looping primitives. The only control flow primitives skip over the singular next instruction. Function calls exist, but only execute a single instruction in a statically-created function state object, rather than actually "calling a function". Also, this statically-created function state object is created at the call site, not the definition site.

Memory cells are 32-bit signed integers and there are 2^16 of them.

```
    inc <immediate> # increment memory cell
    dec <immediate> # decrement memory cell
    ind <immediate> # increment memory cell referred to by memory cell
    ded <immediate> # decrement memory cell referred to by memory cell
    
    toz <immediate> # copy memory cell into memory cell 0
    frz <immediate> # copy memory cell 0 into memory cell
    tod <immediate> # copy memory cell referred to by memory cell into memory cell 0
    frd <immediate> # copy memory cell 0 into memory cell referred to by memory cell
    
    sez <immediate> # copy immediate into memory cell 0
    
    pri <immediate> # print memory cell as unicode codepoint
    cal <name>      # call function
    may <immediate> # jump over next instruction if memory cell is 0
    nmy <immediate> # jump over next instruction if memory cell is not 0
    def <name>      # begin function definition
```

# Hello World

TODO

# Brainfuck interpreter

See `main.fl`. Note the comment at the bottom of the file.
