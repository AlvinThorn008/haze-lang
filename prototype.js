const opList = [
    "(",
    ")",
    "+",
    "-",
    "*",
    "/"
];

const isInt = (char) => !isNaN(parseInt(char));

const tokenise = (source) => {

    const tokens = [];

    for (let i = 0; i < source.length; i++) {
        const char = source[i];

        if (['\n', '\t', ' ', '\r'].includes(char)) continue;

        if (isInt(char)) {
            let token = char;

            while (true) {
                const next = source[i += 1];
                if (isInt(next)) {
                    token += next;
                } else {
                    i -= 1;
                    break;
                }
            }

            tokens.push(token);
            continue;
        }

        const opTok = source[i];

        if (opList.includes(opTok)) { tokens.push(opTok) }
        else { tokens.push("invalid") }
        
    }

    return tokens;
}

console.log(tokenise("5634 / 34 + 23 * 12 ** lop"));

/*
    expr -> term (( '+' | '-' ) term)*
    term -> factor (( '*' | '/' ) factor)*
    factor -> '(' expr ')' | int

    S -> Expr
    Expr -> Term AddOp Term
    Term -> Factor MulOp Factor  
    Factor -> (Expr) | int 
    int
    MulOp -> * | /
    AddOp -> + | -

    5 + 3 / 12 - 34

    0 [5] 6 [+] []

    


 */

/*9 + (3 - 6) * 2

9 + 3 - 6 * 2

9 3 6 2  

+ ( - ) *

9 3 6 2 * ) - ( +






stack + 

*/ 

/*
Expr ::= Term (AddOp Term)*
Term ::= int (MulOp int)*

MulOp ::= '*' | '/'
AddOp ::= '+' | '-'

stack = []

fn expr() {
    let success = match term() {
        true => return false;
    }
    
    loop {
        success = addOp()?;
        success = term()?
    }
   
}
	


 */

const table = new Uint8Array([
    /*
    NT         Row  
    S         0 
    Expr      1
    Term      2
    Factor    3
    AddOp     4
    MulOp     5

    T         Column
    int       0
    +         1
    -         2
    *         3
    /         4
    (         

     */

    /*int  +  -  *  /  (  )*/
        1, 0, 0, 0, 0, 1, 0, // S
        2, 0, 0, 0, 0, 2, 0, // Expr
        3, 0, 0, 0, 0, 3, 0, // Term
        5, 0, 0, 0, 0, 4, 0, // Factor
        0, 6, 7, 0, 0, 0, 0, // AddOp
        0, 0, 0, 8, 9, 0, 0, // MulOp
])

const parse = (src) => {
    const tokens = tokenise(src);

    let i = 0; 

    const tree = [];

    const stack = [];

    let state = 0;

    

    parser2: while (state !== -1) {
        const top = stack[stack.length - 1];
        const tok = tokens[i];

        switch (state) {
            case 0: {
                if (isInt(tok)) {
                    stack.push(tok);
                    state = 1;
                } else {
                    console.log(`Expression must begin with integers. Expected int, found "${tok}" `);
                    break parser2;
                }
                break;
            }
            case 1: {
                if (['*', '/'].includes(tok)) {
                    stack.push(tok);
                    state = 2; // Find rhs
                } else if (['+', '-'].includes(tok)) {
                    if (top == "Term" || tok == undefined) {
                        state = 4; // 
                    } else {
                        state = -1;
                    }
                    stack.push(tok);
                    state = 0;
                }
                break;
            }
            case 2: {
                if (isInt(tok)) {
                    stack.push(tok);
                    state = 3; // reduce
                }
            }
            case 3: {
                
                stack.pop(); 
                stack.pop(); 
                stack.pop();

                stack.push("Term");

                state = 1;
            }
            case 4: {
                stack.pop();
                stack.pop();
                stack.pop();

                stack.push("")
            }
        }
        i++;
    }

    parser: for (let i = 0; i < tokens.length; i++) {
        const tok = tokens[i];

        /*
        5 + 2 - 3 + 2

        5 + TERM

        0 -> 1 [5]
        1 -> 0 [5, +]
        0 -> 1 [5, +, 2]
        1 -> 2 [5, +, 2, *]
        2 -> 1 [5, +, TERM] [3]
        1 -> 2 [5, +, TERM, /]
        2 -> 1 [5, +, TERM] [2]


        Expr -> Term + Expr .
        Expr -> Term + Term .
        Term -> int * Term .
        Term -> int * int .

         */
        let st2 = state;
        switch (state) {
            case 0: {
                if (isInt(tok)) {
                    stack.push(tok);
                    state = 1;
                } else {
                    console.log("ERROR");
                    break parser;
                }
                break;
            }
            case 1: {
                if (['*', '/'].includes(tok)) {
                    stack.push(tok);
                    state = 2;
                } else if (['+', '-'].includes(tok)) {
                    stack.push(tok);
                    state = 0;
                }
                break;
            }
            case 2: {
                if (isInt(tok) || tok == "Term") {
                    stack.push(tok);

                    state = 3;
                } else {
                    console.log("ERROR");
                    break parser;
                }
                break;
            }
            case 3: {
                const op = stack.pop();
                const lhs = stack.pop();

                stack.push("Term");

                state = 1;
            }
        }
        console.log(`${st2} -> ${state} ${stack}`);

        // if (state == 0 && isInt(tok)) {
        //     stack.push(tok);
        //     state = 1; // look for op
        // } else if (state == 1 && ['*', '/'].includes(tok)) {
        //     if (isInt(stack[stack.length - 1])) 
        //     state = 0; // find next int
        // } else if (state == 1 && ['+', '-'].includes(tok)) {
        //     state = 0
        // } 
    }


}

/*
Value -> int | Product | Sum
    Product -> Value ( * | / ) Value
    Sum -> Value ( + | - ) Product

    <Step 1>

    S' -> Value
    Value -> int | Product | Sum
    Product -> Value ( * | / ) Value
    Sum -> Value ( + | - ) Product

    <Step 2>

    S' ->  

    <Step 3>

    S' -> 
        | int 
        | Value ( * | / ) Value 
        | Value ( + | - ) Product

    Value -> 
        | int 
        | Value ( * | / ) Value 
        | Value ( + | - ) Product

    Product -> Value ( * | / ) int
    Sum -> Value ( + | - ) Product

    4 + 5 * 3

    int + 

    <Step 4>

    S' -> 
        | int 
        | Y' Value 
        | Y* Product

    Value -> 
        | int 
        | Y' Value 
        | Y* Product

    Product -> Y' Value
    Sum -> Y* Product

    Y' -> Value ( * | / )
    Y* -> Value ( + | - )

    5 + 9 / 3



    Y* Y' int
 */


/*
    Rule 0 - shift 
    Rule 1 - Multiply */