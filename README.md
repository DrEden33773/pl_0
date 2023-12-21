# PL/0 (aka. PL_0)

## To begin with

This is the `curriculum design` of `Compiler Principle` course in `Nanjing University of Aeronautics and Astronautics` (aka. `NUAA`).

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
\text{<Source Code>} \Longrightarrow  \text{Lexer} \stackrel{Token}{\Longrightarrow} \text{Parser} \stackrel{AST}{\Longrightarrow} \text{CodeGen} \Longrightarrow \text{<PCode>} \stackrel{\textbf{VM}}{\longrightarrow} \set{\text{Result}}
$$

|  Part   |    Analysis List    |
| :-----: | :-----------------: |
|  Lexer  | `Lexical Analysis`  |
| Parser  |  `Syntax Analysis`  |
| CodeGen | `Semantic Analysis` |

## Overview

### Lexer/Tokenizer

This part is extreme easy, I've implemented it in my own hand without using any other tools.

(However, if you'd love to, you could use tools like `flex` or `pest` to generate `lexer/tokenizer` automatically)

### Parser

With the help of `Recursive Descent Algorithm`, `parser` is also not that hard to implement.

However, it's necessary to prove that the given [BNF](#bnf) satisfy the definition of `LL(1)` before implementing `parser` in `Recursive Descent Algorithm`.

Proof will be given later.

### Error Handling

I've adopt the welcomed `panic-mode-liked` error handling strategy for this part, to make sure that the `compiler` could find as many errors as possible in one run, instead of being halted by the first error.

To make sure error could be handled in a `synchros` way, `FIRST-FOLLOW` table is a must (I've build this manually, which could be further improved by using auto-tools).

### Codegen

`AST` to `PCode` code-generator is the default strategy for this part.

I'm working on a `AST` to `Lua-Backend-Adapted-Representation` (LBAR) code-generator as well (not implemented yet).

### Virtual Machine (aka. VM / Interpreter)

Sense `PCode` is the default execution result of `codegen`, the `Simple-PCode-Interpreter` is the default implementation of `Virtual Machine`

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

## Demo

Source code:

```pascal
program fibonacci;

const index := 30;

var return,i,a;
procedure fib(a,x,t);

var sum;
begin
  sum := 0;
  if x<2 then
    return := x
  else
    begin
      call fib(a+1,x-1,t);
      sum := sum+return;
      call fib(a+1,x-2,t);
      sum := sum+return;
      return := sum
    end
end

begin
  i := 1;
  a := 2;
  while i<=index do
    begin
      call fib(a+1,i,0);
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
PCode list:
======================================================================
   0| JMP    0   42
   1| JMP    0    5
   2| STA    1    5
   3| STA    2    4
   4| STA    3    3
   5| INT    0    7
   6| LIT    0    0
   7| STO    0    6
   8| LOD    0    4
   9| LIT    0    2
  10| OPR    0   10
  11| JPC    0   15
  12| LOD    0    4
  13| STO    1    3
  14| JMP    0   41
  15| LOD    0    3
  16| LIT    0    1
  17| OPR    0    2
  18| LOD    0    4
  19| LIT    0    1
  20| OPR    0    3
  21| LOD    0    5
  22| CAL    1    2
  23| LOD    0    6
  24| LOD    1    3
  25| OPR    0    2
  26| STO    0    6
  27| LOD    0    3
  28| LIT    0    1
  29| OPR    0    2
  30| LOD    0    4
  31| LIT    0    2
  32| OPR    0    3
  33| LOD    0    5
  34| CAL    1    2
  35| LOD    0    6
  36| LOD    1    3
  37| OPR    0    2
  38| STO    0    6
  39| LOD    0    6
  40| STO    1    3
  41| OPR    0    0
  42| INT    0    7
  43| LIT    0    1
  44| STO    0    4
  45| LIT    0    2
  46| STO    0    5
  47| LOD    0    4
  48| LIT    0   30
  49| OPR    0   13
  50| JPC    0   65
  51| LOD    0    5
  52| LIT    0    1
  53| OPR    0    2
  54| LOD    0    4
  55| LIT    0    0
  56| CAL    0    2
  57| LOD    0    3
  58| OPR    0   14
  59| OPR    0   15
  60| LOD    0    4
  61| LIT    0    1
  62| OPR    0    2
  63| STO    0    4
  64| JMP    0   47
  65| OPR    0    0
======================================================================
```

- Symbol Table

```txt
Symbol Table:
======================================================================
      name | type   | val  | level  | addr | size | scope_list
======================================================================
     index | const  | 30   | 0      | 3    | 0    | ["main"]
    return | var    | 0    | 0      | 3    | 0    | ["main"]
         i | var    | 0    | 0      | 4    | 0    | ["main"]
         a | var    | 0    | 0      | 5    | 0    | ["main"]
       fib | proc   | 2    | 0      | 6    | 3    | ["main"]
         a | var    | 0    | 1      | 3    | 0    | ["main", "fib"]
         x | var    | 0    | 1      | 4    | 0    | ["main", "fib"]
         t | var    | 0    | 1      | 5    | 0    | ["main", "fib"]
       sum | var    | 0    | 1      | 6    | 0    | ["main", "fib"]
======================================================================
```
