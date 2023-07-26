# Table of contents
- [introduction](#introduction)
- [Types and expressions](#types-and-expressions)
    - [Arithmetic](#arithmetic)
    - [Conditionals](#conditionals)
        - [Local definitions](#local-definitions)
- [Introduction on immutability](#introduction-on-immutability)
- [Pattern matching](#pattern-matching)
- [Recursion](#recursion)

# Introduction

Haskell is: 

- **Functional**: the basic building blocks of programs are functions. The only
looping construct in Haskell is recursion.
- **Pure**: functions in haskell are pure meaning that they don't have side
effects
- **Lazy**: values are only evaluated when they are needed.
- **Strongly type**: every Haskell value and expression has a type. The compiler
checks the types at compile-time and guarantees that no type errors can happen
at runtime.
- **Type inferred**: the compiler is smart enough to deduce the types of most 
programs.
- **Garbage-collected**: Haskell has automatic memory management via garbage 
collection.
- **Compiled**: Haskell programs are compiled to efficient binaries.

# Types and expressions
Almost everything in Haskell is an expression. There are no statements like in
Python, Java or C. An expression has a value and a type. Expressions and types
are written like this: `expression :: type`.

Expressions consist of functions applied to arguments. Functions are called by
placing arguments after the name of the function: `f 1`. Parentheses can be used
to group expressions: `g h (f 1)`. 

Function types are written using `->` syntax: 
```
-- a function of one argument: 
argumentType -> returnType

-- with two arguments:
argumentType -> argument2Type -> returnType

...
```

## Arithmetic
There are two division functions in Haskell, `/` and `div`. The `div` function
does integer division `7 \`div\` 2` while `/` operator does regular division.

> `div` can only be used for division between `Integer` division and `/` is 
   used on decimal types like `Double`.  

## Conditionals
In other languages the `if` statement doesn't have a value, it conditionally
execues other statements. In Haskell `if` is an expression. It has a value,
it selects between two other expressions. It ressembles to the ternary operator
in C or Java.
```Haskell
price = if product == "milk" then 1 else 2
```

> `if` returns a value, therfore you always need an `else`. 

Instead of the usuall `!=` operator not-equals is expressed as `/=` in Haskell.

### Local definitions
Haskell offers two ways of creating local definitions: 
- `let...in` 
- `where` 
```Haskell
circleArea :: Double -> Double
circkeArea r = pi * rsquare
    where pi = 3.1415926
        rsquare = r * r
```

`let...in` is an expression: 
```Haskell
circleArea r = let pi = 3.1415926
                rsquare = r * r
               in pi * rsquare
```

# Introduction on immutability
The values of Haskell definitions can't be changed. Haskell variables aren't 
boxes into which you can put new values. Haskell variable names are a value.

# Pattern matching
A definition of a function can consist of multiple equations. The equations
are matched in order angainst the arguments untill a suitable one is found.
The `_` case matches anything and is therfore used for default cases. When
`_` is used as the first case `ghci` gives a warning about this.

# Recursion
In Haskell looping is implmented with recursion. Recursion is often a usefull
way of thinking about solving harder problems.
```Haskell
factorial :: Int -> Int
factorial 1 = 1
factorial n = n * factorial (n-1)
```

`==>` means to evaluate to 
```Haskell
factorial 3
    ==> 3 * factorial (3-1)
    ==> 3 * factorial 2
    ==> 3 * 2 * factorial 1
    ==> 3 * 2 * 1
    ==> 6
```

note: Have to do th exercise of the first lecture
