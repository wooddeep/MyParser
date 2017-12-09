# The Lexer & Parser for C-Language written in Rust [![Build Status](https://travis-ci.org/sbwtw/MyParser.svg?branch=master)](https://travis-ci.org/sbwtw/MyParser)

## Examples
Print Token List:
```
    let source = "
#include <iostream.h>
int main()
{
    int num = 1;
    if (num == 0)
        return 0;
    else
        return 1;
}
".to_owned();

    let mut lexer = Lexer::new(source.as_bytes());
    while let Some(tok) = lexer.next() {
        println!("{:?}", tok),
    }
```
The output is:
```
Preprocessor("#include <iostream.h>")
KeyWord(Int)
Variable("main")
Bracket(LeftParenthesis)
Bracket(RightParenthesis)
Bracket(LeftCurlyBracket)
KeyWord(Int)
Variable("num")
Operator(Assign)
Number("1")
Semicolon
KeyWord(If)
Bracket(LeftParenthesis)
Variable("num")
Operator(Equal)
Number("0")
Bracket(RightParenthesis)
KeyWord(Return)
Number("0")
Semicolon
KeyWord(Else)
KeyWord(Return)
Number("1")
Semicolon
Bracket(RightCurlyBracket)
```

Print abstract syntax tree
```
    let src = "
int func_add(int a, int b)
{
    return a + b;
}

int main()
{
    return 0;
}
    ";

    let lexer = Lexer::new(src.as_bytes());
    let mut parser = RecursiveDescentParser::new(lexer);
    parser.run();
    parser.dump();
```
The output is:
```
SyntaxTree
  FuncDefine
    Terminal(KeyWord(Int))
    Terminal(Identifier("func_add"))
    FuncArg
      Terminal(KeyWord(Int))
      Terminal(Identifier("a"))
    FuncArg
      Terminal(KeyWord(Int))
      Terminal(Identifier("b"))
    ReturnStmt
      Expr
        Terminal(Identifier("a"))
        Terminal(Operator(Add))
        Terminal(Identifier("b"))
  FuncDefine
    Terminal(KeyWord(Int))
    Terminal(Identifier("main"))
    ReturnStmt
      Terminal(Number("0"))
```

## C-language syntax defines
### 关键字
```
if, else, for, ...
short, int, long, unsigned, ...
```

### 标识符
- number = `\d+`
- identifier = `[a-z][a-z|0-9]*`
- ident = `number` | `identifier`

### 表达式
- expr_opt:
    - `bool_expr`
    - `epsilon`

- expr:
    - `expr` `add_op` `expr_mul`
    - => `expr_mul` `expr_fix` （消除左递归后的产生式，下同）
- expr_fix:
    - `add_op` `expr_mul` `expr_fix` | `epsilon`

- expr_mul:
    - `expr_mul` `mul_op` `expr_factor`
    - => `expr_factor` `expr_mul_fix`
- expr_mul_fix = `mul_op` `expr_factor` `expr_mul_fix` | `epsilon`

- expr_factor = `(` `expr` `)` | `ident`

> 引入的 Tokens
- add_op = `+` | `-`
- mul_op = `*` | `/`
- single_op = `!` | `~`

#### 布尔表达式
> 原始定义
- bool_expr:
    - `bool_expr` `||` `bool_expr`
    - `bool_expr` `&&` `bool_expr`
    - `bool_expr` `equal_op` `bool_expr`
    - `bool_expr` `cmp_op` `bool_expr`
    - `!` `expr`
    - `expr`

> 消除左递归及添加优先级后的定义
- bool_expr:
    - `bool_expr` `||` `bool_expr_and`
    - => `bool_expr_and` `bool_expr_fix`
- bool_expr_fix:
    - `||` `bool_expr_and` `bool_expr_fix` | `epsilon`

- bool_expr_and:
    - `bool_expr_and` `&&` `bool_expr_equal`
    - => `bool_expr_equal` `bool_expr_and_fix`
- bool_expr_and_fix:
    - `&&` `bool_expr_equal` `bool_expr_and_fix` | `epsilon`

- bool_expr_equal:
    - `bool_expr_equal` `equal_op` `bool_expr_cmp`
    - => `bool_expr_cmp` `bool_expr_equal_fix`
- bool_expr_equal_fix:
    - `equal_op` `bool_expr_cmp` `bool_expr_equal_fix` | `epsilon`

- bool_expr_cmp:
    - `bool_expr_cmp` `cmp_op` `bool_expr_factor`
    - => `bool_expr_factor` `bool_expr_cmp_fix`
- bool_expr_cmp_fix:
    - `cmp_op` `bool_expr_factor` `bool_expr_cmp_fix` | `epsilon`

- bool_expr_factor:
    - `!` `bool_expr`
    - `(` `bool_expr` `)`
    - `expr`

> 引入的 Tokens
- cmp_op:
    - `>`
    - `>=`
    - `<`
    - `<=`
- equal_op:
    - `==`
    - `!=`

### 语句

- stmt:
    - `stmt_factor`

- stmt_factor:
    - `stmt_single` `;`
    - `stmt_block`
    - `stmt_control`
    - `;`

- stmt_single
    - `assign_stmt`
    - `break_stmt`
    - `return_stmt`

- stmt_control
    - `if_stmt`
    - `while_loop`
    - `for_loop`

- stmt_list:
    - `stmt` `stmt_list` | `epsilon`

- stmt_block:
    - `{` `stmt_list` `}`

- assign_stmt:
    - `left_value` `=` `right_value`

- if_stmt:
    - `if` `(` `bool_expr` `)` `stmt` `else` `stmt`

- while_loop:
    - `while` `(` `bool_expr` `)` `stmt`

- for_loop:
    - `for` `(` `expr_opt` `;` `expr_opt` `;` `expr_opt` `)` `stmt`

- break_stmt:
    - `break`

- return_stmt:
    - `return` `return_expr`

- return_expr:
    - `bool_expr`
    - `epsilon`

- left_value:
    - `identifier`
- right_value:
    - `bool_expr`

### 声明 & 定义
#### 变量定义
- variable_define:
    - `variable_type` `variable_def_list`

- variable_def_list:
    - `identifier` `;`
    - `identifier` `,` `variable_def_list`

- variable_type:
    - `variable_prefix` `variable_suffix`
    - `float`
    - `double`

- variable_prefix:
    - `unsigned`
    - `signed`
    - `long`
    - `long long`

- variable_suffix:
    - `int`

#### 函数声明
- func_declare:
    - `func_ret_type` `func_name` `(` `func_arg_list` `)` `;`

- func_name
    - `identifier`

- func_ret_type:
    - `type`

- func_arg_list:
    - `func_arg` `func_arg_list_tail`
    - `epsilon`

- func_arg_list_tail:
    - `,` `func_arg` `func_arg_list_tail`
    - `epsilon`

- func_arg:
    - `func_arg_type` `func_arg_name`

- func_arg_type:
    - `type`

- func_arg_name:
    - `identifier`

#### 结构体定义
- struct_define:
    - `struct` `{` `struct_vars` `}` `;`
    - `struct` `identifier` `{` `struct_vars` `}` `;`

- struct_vars
    - `struct_var` `;`
    - `struct_var` `struct_vars`
    - `epsilon`

- struct_var
    - `variable_define`

#### 函数定义
- function:
    - `func_ret_type` `func_name` `(` `func_arg_list` `)` `{` `func_body` `}`

- func_body:
    - `stmt_list`

