# BNF

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
              | read (<id> {, <id>})
              | write (<exp> {, <exp>})
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
