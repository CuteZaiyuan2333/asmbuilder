//! AI 生成的 Symphony 汇编示例。
//!
//! 这些程序只依赖 `crate::backends::symphony` 现有 API，不修改后端。
//! 目的不是“写出最优汇编”，而是拿几个不同形态的算法来试：
//! - 直线代码（无跳转）是否顺手；
//! - 立即数 / 寄存器 / Load / Store 的组合是否好写；
//! - 一进入真实控制流（label 跳转）时，生成器是否还可用。
//!
//! 运行演示：
//! `cargo test -p asmbuilder print_demo -- --nocapture`

/// 用 Rust 宏拼出“一个 label 一个 Block”的汇编程序。
///
/// 这是 aigen 里唯一的元编程糖：label 名直接复用标识符，
/// 每条指令仍然是普通的 Rust 表达式。
#[macro_export]
macro_rules! asm_program {
    ($($label:ident: [$($insn:expr),* $(,)?]),* $(,)?) => {{
        let blocks = vec![
            $(
                $crate::aigen::dsl::block(stringify!($label), vec![$($insn),*])
            ),*
        ];
        $crate::aigen::dsl::build_blocks(blocks)
    }};
}

pub mod array_sum;
pub mod bubble_sort;
pub mod dsl;
pub mod fibonacci;
pub mod gcd;
pub mod polynomial;

pub type ProgramResult = Result<String, &'static str>;

pub fn generate_all() -> Vec<(&'static str, ProgramResult)> {
    vec![
        ("fibonacci", fibonacci::build()),
        ("polynomial_horner", polynomial::build()),
        ("array_sum", array_sum::build()),
        ("gcd", gcd::build()),
        ("bubble_sort", bubble_sort::build()),
    ]
}

pub fn run_all() {
    for (name, result) in generate_all() {
        println!("; ===== {name} =====");
        match result {
            Ok(asm) => print!("{asm}"),
            Err(reason) => println!("// build failed: {reason}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result_of(name: &str) -> ProgramResult {
        generate_all()
            .into_iter()
            .find(|(program, _)| *program == name)
            .map(|(_, result)| result)
            .unwrap_or_else(|| panic!("unknown program: {name}"))
    }

    #[test]
    fn all_programs_build() {
        for name in [
            "fibonacci",
            "polynomial_horner",
            "array_sum",
            "gcd",
            "bubble_sort",
        ] {
            assert!(result_of(name).is_ok(), "{name} should build");
        }
    }

    #[test]
    fn memory_program_uses_both_address_modes() {
        let asm = result_of("array_sum").expect("array_sum should build");
        assert!(asm.contains("store8 "), "expected a register-address store:\n{asm}");
        assert!(asm.contains("load8i "), "expected an immediate-address load:\n{asm}");
    }

    #[test]
    fn label_jumps_build_now() {
        let gcd = result_of("gcd").expect("gcd should build");
        assert!(gcd.contains("gcd_loop"), "expected a label jump target:\n{gcd}");

        let sort = result_of("bubble_sort").expect("bubble_sort should build");
        assert!(sort.contains("bs_inner"), "expected a label jump target:\n{sort}");
    }

    #[test]
    fn print_demo() {
        run_all();
    }
}
