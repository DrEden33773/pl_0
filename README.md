# PL/0 (aka. PL_0)

> ## ❤️ Please give me a `Star` / `Follow` if you like this project! ❤️

## To begin with

This is the `curriculum design` of `Compiler Principle` course in `Nanjing University of Aeronautics and Astronautics` (
aka. `NUAA`).

## Introduction

`PL/0` is a `subset language` of `Pascal`.

This is a simple `Rust` implementation of `PL/0` compiler.

## BNF

```bnf
<prog> -> program <id> ; <block>
<block> -> [<const-decl>][<var-decl>][<proc>]<body>
<const-decl> -> const <const> {, <const>} ;
<const> -> <id> := <integer>
<var-decl> -> var <id> {, <id>} ;
<proc> -> procedure <id> ([<id> {, <id>}]) ; <block> {; <proc>}
<body> -> begin <statement> {; <statement>} end
<statement> -> <id> := <exp>
              | if <l-exp> then <statement> [else <statement>]
              | while <l-exp> do <statement>
              | call <id> ([<exp> {, <exp>}])
              | <body>
              | read (<id> {, <id>})
              | write (<exp> {, <exp>})
<l-exp> -> <exp> <lop> <exp> | odd <exp>
<exp> -> [+|-] <term> {<aop> <term>}
<term> -> <factor> {<mop> <factor>}
<factor> -> <id> | <integer> | (<exp>)
<lop> -> = | <> | < | <= | > | >=
<aop> -> + | -
<mop> -> * | /
<id> -> <letter> {<letter> | <digit>}
<integer> -> <digit> {<digit>}
<letter> -> a | b | ... | z | A | B | ... | Z
<digit> -> 0 | 1 | ... | 9
```

## Structure

$$
\set{Source Code} \Longrightarrow \textbf{Lexer} \stackrel{Token}{\Longrightarrow} \textbf{Parser} \stackrel{AST}{\Longrightarrow} \textbf{CodeGen} \Longrightarrow \set{PCode} \longrightarrow \textbf{VM} \longrightarrow \set{Result}
$$

|  Part   |    Analysis List    |
|:-------:|:-------------------:|
|  Lexer  | `Lexical Analysis`  |
| Parser  |  `Syntax Analysis`  |
| CodeGen | `Semantic Analysis` |

## Overview

### Lexer/Tokenizer

This part is extreme easy, I've implemented it in my own hand without using any other tools.

(However, if you'd love to, you could use tools like `flex` or `pest` to generate `lexer/tokenizer` automatically)

### Parser

With the help of `Recursive Descent Algorithm`, `parser` is also not that hard to implement.

However, it's necessary to prove that the given [BNF](#bnf) satisfy the definition of `LL(1)` before implementing
`parser` in `Recursive Descent Algorithm`.

Proof will be given later.

### Error Handling

I've adopted the welcomed `panic-mode-liked` error handling strategy for this part, to make sure that the `compiler`
could find as many errors as possible in one run, instead of being halted by the first error.

To make sure error could be handled in a `synchronous` way, `FIRST-FOLLOW` table is a must (I've built this manually,
which could be further improved by using auto-tools).

### Codegen

`AST` to `PCode` code-generator is the default strategy for this part.

I'm working on a `AST` to `Lua-Backend-Adapted-Representation` (LBAR) code-generator as well (not implemented yet).

### Virtual Machine (aka. VM / Interpreter)

Sense `PCode` is the default execution result of `codegen`, the `Simple-PCode-Interpreter` is the default implementation
of `Virtual Machine`

Still, I'm trying to implement a `Lua-VM-Liked-VM` for `LBAR`

## Feasibility Analysis

### Proof: [BNF](#bnf) is `LL(1)`

To satisfy this, 3 conditions should be met:

$$
\begin{align*}
\text{Condition 1} &~\dots~ \text{No \textit{left recursion pattern} detected in the \textit{grammar}} \\
\text{Condition 2} &~\dots~ \forall A \in V_N (A \rightarrow \alpha_1 | \alpha_2 | \dots | \alpha_n) \Rightarrow First(\alpha_i) \cap First(\alpha_j) = \Phi ~ (i \ne j) \\
\text{Condition 3} &~\dots~ \forall A \in V_N (\epsilon \in First(A)) \Rightarrow First(A) \cap Follow(A) = \Phi
\end{align*}
$$

Now, let's prove them one by one!

#### Condition 1 ~ No _left recursion pattern_ detected in the _grammar_

After having a glance of the given [BNF](#bnf), we could easily prove that:

$$
\forall A \in V_N (A \rightarrow B ~\wedge~ B \in V_N ) \Rightarrow A \ne B
$$

Which means that, there's no _left recursion pattern_ detected in the _grammar_.

#### Condition 2

This could be easy, with the reference of [BNF](#bnf) and [first_follow_table](./src/parser/synchronizer/tables.rs)

#### Condition 3

Just the same as `Condition 2`

## Fibonacci Demo

Source code:

```pascal

program fibonacci;

const index := 30;

var return,i,a;

procedure fib(a,x);

var sum;
begin
  sum := 0;
  if x<2 then
    return := x
  else
    begin
      call fib(a+1,x-1);
      sum := sum+return;
      call fib(a+1,x-2);
      sum := sum+return;
      return := sum
    end
end

begin
  i := 1;
  a := 2;
  while i<=index do
    begin
      call fib(a+1,i);
      write(return);
      i := i+1
    end
end

```

Result:

- Console

```txt
1
1
2
3
5
8
13
21
34
55
89
144
233
377
610
987
1597
2584
4181
6765
10946
17711
28657
46368
75025
121393
196418
317811
514229
832040
```

- PCode

```txt
PCode List:
======================================================================
   0| JMP    0   39
   1| JMP    0   4
   2| STA    1   4
   3| STA    2   3
   4| INT    0   6
   5| LIT    0   0
   6| STO    0   5
   7| LOD    0   4
   8| LIT    0   2
   9| OPR    0   10
  10| JPC    0   14
  11| LOD    0   4
  12| STO    1   3
  13| JMP    0   38
  14| LOD    0   3
  15| LIT    0   1
  16| OPR    0   2
  17| LOD    0   4
  18| LIT    0   1
  19| OPR    0   3
  20| CAL    1   2
  21| LOD    0   5
  22| LOD    1   3
  23| OPR    0   2
  24| STO    0   5
  25| LOD    0   3
  26| LIT    0   1
  27| OPR    0   2
  28| LOD    0   4
  29| LIT    0   2
  30| OPR    0   3
  31| CAL    1   2
  32| LOD    0   5
  33| LOD    1   3
  34| OPR    0   2
  35| STO    0   5
  36| LOD    0   5
  37| STO    1   3
  38| OPR    0   0
  39| INT    0   7
  40| LIT    0   1
  41| STO    0   4
  42| LIT    0   2
  43| STO    0   5
  44| LOD    0   4
  45| LIT    0   30
  46| OPR    0   13
  47| JPC    0   61
  48| LOD    0   5
  49| LIT    0   1
  50| OPR    0   2
  51| LOD    0   4
  52| CAL    0   2
  53| LOD    0   3
  54| OPR    0   14
  55| OPR    0   15
  56| LOD    0   4
  57| LIT    0   1
  58| OPR    0   2
  59| STO    0   4
  60| JMP    0   44
  61| OPR    0   0
======================================================================
```

- Symbol Table

```txt
Symbol Table:
======================================================================
      name | type   | val  | level  | addr | size | scope_list
======================================================================
     index | const  | 30   | 0      | 3    | 0    | ["#"]
    return | var    | 0    | 0      | 3    | 0    | ["#"]
         i | var    | 0    | 0      | 4    | 0    | ["#"]
         a | var    | 0    | 0      | 5    | 0    | ["#"]
       fib | proc   | 2    | 0      | 6    | 2    | ["#"]
         a | var    | 0    | 1      | 3    | 0    | ["#", "fib"]
         x | var    | 0    | 1      | 4    | 0    | ["#", "fib"]
       sum | var    | 0    | 1      | 5    | 0    | ["#", "fib"]
======================================================================
```

## Error Handling Demos

As is mentioned follow, this implementation of pl/0 compiler has a complete error handling strategy, which means that it
could find as many errors as possible in one run, instead of being halted by the first error.

Here are some simple demos:

### Syntax Error (may coexists with `Lexical Error`)

- src

```pascal
program ;
var a, b, c;
begin
  a    1;
  b :=  ;
  é : 3;
  if 1 = 1 then
    write(1
  else
    write 0);
  write a + b + c;
  wrçte(1)
end
```

- console

```txt
SyntaxError{ Line: 1, Col: 9 }
  | ~~ Expected <id> field, but not found!

SyntaxError{ Line: 4, Col: 8 }
  | ~~ Expected `:=`, but got `Integer(1)`

SyntaxError{ Line: 5, Col: 9 }
  | ~~ Expected `<id>` / `<integer>` / `(<exp>)` field, but got an unmatchable token `;`

LexicalError{ Line: 6, Col: 3 }
  | ~~ 'é' is not an ASCII character

LexicalError{ Line: 6, Col: 5 }
  | ~~ ':' is an undefined sign, did you mean ':='?

SyntaxError{ Line: 6, Col: 7 }
  | ~~ Expected `:=`, but got `Integer(3)`

SyntaxError{ Line: 6, Col: 7 }
  | ~~ Expected <statement> field, but not found!

SyntaxError{ Line: 9, Col: 6 }
  | ~~ Expected `)`, but got `Else`

SyntaxError{ Line: 10, Col: 11 }
  | ~~ Expected `(`, but got `Integer(0)`

SyntaxError{ Line: 11, Col: 9 }
  | ~~ Expected `(`, but got `Identifier("a")`

SyntaxError{ Line: 11, Col: 18 }
  | ~~ Expected `)`, but got `;`

LexicalError{ Line: 12, Col: 5 }
  | ~~ 'ç' is not an ASCII character

SyntaxError{ Line: 12, Col: 7 }
  | ~~ Expected `:=`, but got `Identifier("te")`

SyntaxError{ Line: 12, Col: 7 }
  | ~~ Expected <statement> field, but not found!

thread 'main' panicked at src/parser/mod.rs:149:7:
|> Errors above occurred (during `parsing`), compiling stopped ... <|

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

### Semantic Error

#### Duplicated Definition

- src

```pascal
program MultiDef;

var a, a, a, a;

procedure proc();
begin
  write(1)
end;

procedure proc();
begin
  write(2)
end

begin
  write(1)
end
```

- console

```txt
SemanticError{ Line: 3, Col: 8 }
  | ~~ `a` is defined before

SemanticError{ Line: 3, Col: 11 }
  | ~~ `a` is defined before

SemanticError{ Line: 3, Col: 14 }
  | ~~ `a` is defined before

SemanticError{ Line: 10, Col: 14 }
  | ~~ `proc` is defined before

thread 'main' panicked at src/translator/mod.rs:116:7:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### Undefined

- src

```pascal
program undef;
begin
  a := 1;
  b := 2;
  write(c)
end
```

- console

```txt
SemanticError{ Line: 3, Col: 3 }
  | ~~ `a` is undefined

SemanticError{ Line: 4, Col: 3 }
  | ~~ `b` is undefined

SemanticError{ Line: 5, Col: 9 }
  | ~~ `c` is undefined

thread 'main' panicked at src/translator/mod.rs:73:7:
|> Errors above occurred (during `translation/codegen`), compiling stopped ... <|

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### `args_list.length` cannot match with definition(signature)

- src

```pascal
program WrongArgsListLength;

var a;

procedure proc();
begin
  write(1)
end;

procedure procc(x, t, z);
begin
  write(1)
end

begin
  call proc(1, 1, 1);
  call procc(3)
end
```

- console

```txt
SemanticError{ Line: 16, Col: 11 }
  | ~~ `proc` expects 0 args, but received 3

SemanticError{ Line: 17, Col: 12 }
  | ~~ `procc` expects 3 args, but received 1

thread 'main' panicked at src/translator/mod.rs:73:7:
|> Errors above occurred (during `translation/codegen`), compiling stopped ... <|
```

#### Assign to `const` / `procedure`

- src

```pascal
program AssignToConstProc;
const i := 1;

procedure proc();
begin
  write(i)
end

begin
  i := 16;
  proc := 16
end
```

- console

```txt
SemanticError{ Line: 10, Col: 3 }
  | ~~ `i` is not a variable

SemanticError{ Line: 11, Col: 6 }
  | ~~ `proc` is not a variable

thread 'main' panicked at src/translator/mod.rs:73:7:
|> Errors above occurred (during `translation/codegen`), compiling stopped ... <|
```
