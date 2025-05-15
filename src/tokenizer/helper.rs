pub mod keyword {
    #[derive(Debug)]
    pub enum KeywordType {
        Exit,
    }
}

pub mod expression {
    pub mod literal {
        pub enum LiteralType {
            IntegerLiteral
        }
    }
    pub enum ExpressionType {
        Literal(literal::LiteralType),
    }
}