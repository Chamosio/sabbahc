use crate::tokenizer;

pub fn compile(input: &Vec<tokenizer::StatementAST>) -> String {
    String::from(
        "global _start\n_start:\n    mov rax, 60\n    mov rdi, 255\n    syscall\n"
    )
}