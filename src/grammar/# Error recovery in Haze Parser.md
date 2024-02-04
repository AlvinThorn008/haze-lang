# Error recovery in Haze Parser

## Top level declarations

### Module

    ```ebnf
    Module = module ident '{' TopLevelDeclaration* '}' ;
    ```

- If the identifier is missing, it may be filled with an empty token and parsing should continue.

- Top level declarations that fail to parse should be discarded from the list.

- Braces are tricky. If the left brace is missing, we should recover, either finding a right brace or leading token. At the moment, the parser doesn't track brace match globally so it may well lead to cascading errors

### Import declaration

    ```ebnf
    ImportDeclaration = import SymbolPathExpr ';' ;

    SymbolPathExpr = ident ('.' ident)* ;
    ```

- The symbol path MUST be valid. On the first syntax error, bail out and do not attempt to construct an import declaration.

### Struct declaration

    ```ebnf
    StructDeclaration = struct ident '{' StructFields '}' ;

    StructFields = StructField (',' StructField)* ','? ;

    StructField = ident ':' TypeExpr ;
    ```

- If the identifier is missing, it may be filled with an empty token and parsing should continue.

- 

## Statements

### Variable declaration

    ```ebnf
    VariableDeclaration = let ident (':' TypeExpr)? ('=' Expr)? ';' ;
    ```

If