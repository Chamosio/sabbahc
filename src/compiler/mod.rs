use std::process::exit;

use crate::tokenizer;

pub fn compile(input: &Vec<tokenizer::StatementAST>, timestamp: bool, org: &String) -> String {
    let mut output = String::from("global _start\n_start:\n");
    for statement in input {
        for token in &statement.children {
            if let tokenizer::TokenType::Keyword(keyword_type) = &token.token_type {
                match keyword_type {
                    tokenizer::KeywordType::Exit => {
                        if statement.children.len() == 1 {
                            output.push_str("    mov rax, 60\n    mov rdi, 0\n    syscall\n");
                        } else if statement.children.len() == 2 {
                            output.push_str("    mov rax, 60\n    mov rdi, ");
                            output.push_str(&statement.children[1].value.as_ref().unwrap());
                            output.push_str("\n    syscall\n");
                        } else {
                            println!("ERROR: Invalid exit statement");
                            exit(7)
                        }
                    }
                }
            }
        }
    }
    
    output.push_str("    mov rax, 60\n    mov rdi, 0\n    syscall\n");
    if timestamp {
        output.push_str(format!("    ; File {} compiled by sabbahc on ", org).as_str());
        let now = chrono::Local::now();
        output.push_str(&now.format("%Y-%m-%d %H:%M:%S").to_string());
        output.push_str("\n");
    }
    return output;
}