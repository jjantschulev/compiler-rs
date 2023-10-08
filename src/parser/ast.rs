use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ImportStatement { path: String, imports: Vec<String> },
}

// impl Parsable for Statement {
//     fn parse(lexer: &mut Peekable<Lexer>) -> Result<Self, ParseError> {
//         match lexer.next().map(|t| t.token) {
//             None => return Err(ParseError::new("Unexpected EOF".to_string())),
//             Some(TokenType::Keyword(Keyword::Import)) => {
//                 let idents = parse_ident_list(lexer)?;
//                 expect_token(lexer, AbstractTokenType::Keyword(Keyword::From))?;

//                 match lexer.next() {
//                     Some(Token {
//                         token: TokenType::Literal(Literal::String(path)),
//                         ..
//                     }) => Ok(Statement::ImportStatement {
//                         path,
//                         imports: idents,
//                     }),
//                     tok => Err(ParseError::new(format!(
//                         "Unexpected token {:?}, expected string literal",
//                         tok
//                     ))),
//                 }
//             }
//             tok => {
//                 return Err(ParseError::new(format!(
//                     "Unexpected token {:?}, expected import statement",
//                     tok
//                 )))
//             }
//         }
//     }
// }

// fn expect_token(
//     lexer: &mut Peekable<Lexer>,
//     token: AbstractTokenType,
// ) -> Result<Token, ParseError> {
//     match lexer.next() {
//         None => Err(ParseError::new("Unexpected EOF".to_string())),
//         Some(tok) => {
//             if tok.token.is_of_type(&token) {
//                 Ok(tok)
//             } else {
//                 Err(ParseError::new(format!(
//                     "Unexpected token {:?}, expected {:?}",
//                     tok.token, token
//                 )))
//             }
//         }
//     }
// }

// fn parse_ident_list(lexer: &mut Peekable<Lexer>) -> Result<Vec<String>, ParseError> {
//     let mut idents = Vec::new();

//     loop {
//         match lexer.peek() {
//             None => return Err(ParseError::new("Unexpected EOF".to_string())),
//             Some(tok) => match &tok.token {
//                 TokenType::Ident(ident) => {
//                     idents.push(ident.clone());
//                     lexer.next();
//                 }
//                 TokenType::SpecialChar(SpecialChar::Comma) => {
//                     lexer.next();
//                 }
//                 _ => break,
//             },
//         }
//     }

//     Ok(idents)
// }

// enum Statement {
//     Expr(Expr)
//     VarDecl
// }

// struct Block {
//     statements: Vec<Statement>,
// }

// enum Ast {
//     Block(Block),
// }
