# XL Language Specification (draft v0.3)

> NOTE: This is **work in progress**.  Sections prefixed with ▶ will be fleshed out in subsequent PRs.

## 0. Philosophy
XL is a *compiler-construction* language.  It must be:
* **Deterministic** – no hidden order-of-evaluation surprises.
* **Composable** – user-defined AST nodes and passes are first-class.
* **Safe by default** – memory & type soundness.
* **Powerful** – pattern-rewrite DSL, SSA backend, LLVM out-of-the-box.

## 1. Lexical grammar (Unicode-aware)
```
identifier     ::=  ( ALPHA | '_' ) { ALNUM | '_' }
intLiteral     ::=  DIGIT { DIGIT }
stringLiteral  ::=  '"' … '"'
keyword        ::=  'fn' | 'let' | 'return' | 'ast' | 'module' | 'import' | 'export' | 'match' | 'rewrite' | 'pass' | 'if'
operator       ::=  '+' | '-' | '*' | '/' | '->' | '=>' | '='
```
▶ Full token table in docs/grammar.ebnf

## 2. Concrete syntax
### 2.1 Modules
```
File ::= { ImportDecl | AstDecl | Function | ModuleDecl } EOF
ImportDecl ::= 'import' Path ';'
ExportDecl ::= 'export' Item
ModuleDecl ::= 'module' ident '{' { Item } '}'
```

### 2.2 AST declarations
```
ast Binary(op: Token, lhs: Expr, rhs: Expr);
```
*Compiles to a struct `Binary` and pattern `Binary(op, lhs, rhs)`*

### 2.3 Pattern-matching & rewrite
See section 5.

## 3. Type system
Types are *nominal*, parametric and support ADTs.
```
Type   ::= SimpleType | GenericType | StructType | EnumType | Pointer | FuncType | TupleType
GenericType ::= ident '<' Type { ',' Type } '>'
StructType  ::= 'struct' ident '{' FieldList '}'
```
▶ Inference rules in §3.3.

## 4. Intermediate Representations
* **HIR** – typed AST after desugaring.
* **MIR (SSA)** – in `ssa.rs`, each function is a set of basic blocks.
* **LLVM IR** – generated via inkwell.

## 5. Pattern engine
▶ Formal semantics of `rewrite` decision-tree.

## 6. Pass manager & phases
▶ Build graph, support for incremental & parallel.

## 7. Standard library layout
▶ `core`, `collections`, `parse`, `llvm`.

## 8. Error handling & diagnostics
▶ Span model, error codes.

## A. Reference grammar
See `grammar.ebnf`.