# PL_0

## Introduction

`PL_0` is a `subset language` of `Pascal`.

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

## Overview

### Lexer/Tokenizer

This part is extreme easy, I've implemented it in my own hand without using any other tools.

(However, if you'd love to, you could use tools like `flex` or `pest` to generate `lexer/tokenizer` automatically)

### Parser

With the help of `Recursive Descent Algorithm`, `parser` is also not that hard to implement.

However, it's necessary to prove that the given [BNF](#bnf) satisfy the definition of `LL(1)` before implementing `parser` in `Recursive Descent Algorithm`.

Proof will be given later.

### TODO: Error Handling

I'll adapt the welcomed `panic-mode` error handling strategy for this part, to make sure that the `compiler` could find as many errors as possible in one run, instead of being halted by the first error.

## Feasibility Analysis

### Proof: [BNF](#bnf) is `LL(1)`

To satisfy this, 3 conditions should be met:

1. $$ \text{No \textit{left recursion pattern} detected in the \textit{grammar}} $$
2. $$ \forall A \in V_N (A \rightarrow \alpha_1 | \alpha_2 | \dots | \alpha_n) \Rightarrow First(\alpha_i) \cap First(\alpha_j) = \Phi ~ (i \ne j) $$
3. $$ \forall A \in V_N (\epsilon \in First(A)) \Rightarrow First(A) \cap Follow(A) = \Phi $$

Now, let's prove them one by one!

#### Condition#1 ~ No _left recursion pattern_ detected in the _grammar_

After having a glance of the given [BNF](#bnf), we could easily prove that:

$$
\forall A \in V_N (A \rightarrow B ~\wedge~ B \in V_N ) \Rightarrow A \ne B
$$

Which means that, there's no _left recursion pattern_ detected in the _grammar_.

#### Condition#2

TODO:

#### Condition#3

TODO:
