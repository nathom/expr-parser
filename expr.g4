grammar Arithmetic;

expr : INT rest_of_expr
     | '(' expr ')'
     ;

rest_of_expr : times_div_term '+' expr
             | times_div_term '-' expr
             |
             ;

times_div_term  : '*' expr
                | '/' expr
                |
                ;
// Lexer rules
INT :   [0-9]+ ;
WS  :   [ \t\r\n]+ -> skip ;
