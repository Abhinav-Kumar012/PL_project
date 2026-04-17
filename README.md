# PL project

## Problem statement
Exploring Meta-Programming \
TYPE: (study) \
Meta-programming is the art of writing code that writes or manipulates other code (including
itself).
- Analyze the meta-programming features of languages like Ruby and Lisp.
- Investigate the implications of meta-programming on programming language design, focusing on syntax, semantics, and language features that facilitate or hinder meta-programming
techniques.
- Meta-programming relies on reflective capabilities or meta-object protocols, which allow
programs to inspect and modify their own structure and behavior.
- Examine how meta-programming interacts with the type systems of programming languages, including challenges related to type safety and type inference in the context of
dynamically generated code

---

## Metaprogramming

- Programming where data and code are the same.
- A rigorous way to describe it, at least in case of Lisp, is that it is programming ASTs.

## Lisp and MetaProgramming
Lisp uses something called homoiconicity- where code and data share the same expressions called S-expressions.

S-expressions are literally the ASTs of our program. Because Lisp uses pre-order notation to describe functions, the ASTs are linear linked lists instead of tree. 
And we program by writing these lists.

When this list based AST is passed to eval, only then is the function evaluated.

## Type Issues

- Fundamental tussle between MetaProgramming and Typing- the circular loop
- The two-phase compilation of templates in C++, the notoriety of second phase bugs and C++ Concepts
- Turing Completeness as a reducible problem


---
