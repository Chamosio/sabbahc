pub mod keyword {
    #[derive(Debug, Clone)]
    pub enum KeywordType {
        Exit,
    }
}

pub mod expression {
    pub mod literal {
        #[derive(Debug, Clone)]
        pub enum LiteralType {
            IntegerLiteral
        }
    }
    #[derive(Debug, Clone)]
    pub enum ExpressionType {
        Literal(literal::LiteralType),
    }
}