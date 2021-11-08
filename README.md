# FunkshunL
FunkshunL is an esoteric programming language with no flow control and broken function call semantics.

FunkshunL is quasi-turing-complete and has a working brainfuck interpreter (see main.fl).

# Design

FunkshunL is a direct memory access language with no looping primitives. The only control flow primitives skip over the singular next instruction. Function calls exist, but only execute a single instruction in a statically-created function state object, rather than actually "calling a function". Also, this statically-created function state object is created at the ***call site***, not the definition site.

This means that if you want a function to actually call, you need to write `cal myfunc` however many times there are instructions in the function... except that wouldn't work because each ***call site*** has its own function state object, so you have to wrap it in another function first and then call that instead. (Functions loop when they finish running, except for main, which does not loop.)

The purpose of this is to make it so that you can't actually loop.

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

# Brainfuck interpreter

See `main.fl`. Note the comment at the bottom of the file.

# Hello World

```
def main
sez 72
pri 0
sez 101
pri 0
sez 108
pri 0
pri 0
sez 111
pri 0
sez 32
pri 0
sez 119
pri 0
sez 111
pri 0
sez 114
pri 0
sez 108
pri 0
sez 100
pri 0
sez 33
pri 0
```
