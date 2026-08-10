# BOMP: BOOF LANG Compiler

BOOF LANG: Bad Objectively Obsolete Functional Language

Take the best parts of functional programming and remove them

All primitives are words
All aggregates have a fixed width, called packs
Algebraic datatypes have variants of different width, can be recursive

Example

structure list = Cons of 1 * list | Empty
structure tree = Node of 1 * list * list | Empty
structure pair = 2

Floats and ints are inferred based on operators. 
+, -, *, /, etc will use unsigned int semantics
+., -., *., /., etc use unsigned float semantics
~/, ~%, ~>> will use signed int semantics
~/. is unsigned float, 

words are stored on the stack, all other types are stored on the heap

We use a borrow checker to avoid garbage collection and memory management?

We don't have types. There are structures and packs, but no "types"

functions are also first class

Globals cannot be dynamic. Thus, we should be able to do constant propagation within the compiler to determine their value at compile time. To do this correctly, we need to check that no constants form a cycle

KNOWN BUGS:
scope (interner fix, should have new, intern, resolve? unsure)

## Reserved Keywords
`fun`, `in`, `pack`, `word`

# STEPS
1. Lexer
2. Parser
3. Struct checker/inference
4. Borrow checker
5. AST to BIR
6. BIR Type Checker
7. BIR to LLVM

# MVP
- Functions over words
- Globals

# Features in development
- Float support
- Pack handling - Determine
- Higher-order functions
- Linear types?

# Eventual features
- Functions over structures
- Structures in general
- Higher-order functions
- Partial application
- Generics
- Strings & Macros
- Features for running on GPU
- Type-based verification - Predicate is typing for vars, verify some property

# Compilation Strategy

use BIR, an IR that we can then do optimizations on

BIR is like LLVM, but untyped.

BIR then compiles to LLVM

TYPES
- i64
- ptr
- f_ptr
- pack[n] - pack of width n, can contain any of the above

OPERANDS
- null - used for ptrs (None, Empty, etc.)
- [0-9]+ - words
- %IDENT - local identifier
- @IDENT - global identifier [v2]

INSTRUCTIONS
- %L = BOP word OP1, OP2	word x word → word
- %L = alloca S	- → S*
- %L = load S* OP	S* → S
- store S OP1, S* OP2	S x S* → void
- %L = icmp CND S OP1, OP2	S x S → word
- %L = call S1 OP1(S2 OP2, ..., SN OPN)	S1(S2, ..., SN)* x S2 x ... x SN → S1
- call void OP1(S2 OP2, ... ,SN OPN)	void(S2, ..., SN)* x S2 x ... x SN → void
- %L = getelementptr T1* OP1, i32 OP2, ..., i32 OPN  	T1* x word x ... x word -> GEPTY(T1, OP1, ..., OPN)* [NEED TO REWORK]