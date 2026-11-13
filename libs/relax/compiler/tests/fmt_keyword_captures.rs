// A captured format argument is a *name*, resolved by the format machinery, never by the parser:
// rustc's `format_args!` capture emits an `Ident` token whatever the lexer would call the spelling
// (rustc_builtin_macros format.rs). So `{ref}`, `{in}` and `{type}` capture `r#ref`, `r#in` and
// `r#type` bindings — sqlparser 0.62 writes `{in}`/`{type}` and iceberg writes `{ref}`.
pub fn with_raw_ident_bindings() -> String {
    let r#ref = "main";
    let r#type = 7u32;
    format!("Branch `{ref}` not found, kind {type}")
}

pub enum E {
    Pos { expr: u32, r#in: u32 },
}

impl std::fmt::Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            E::Pos { expr, r#in } => write!(f, "POSITION({expr} IN {in})"),
        }
    }
}

#[test]
fn keyword_spelled_captures_resolve() {
    assert_eq!(with_raw_ident_bindings(), "Branch `main` not found, kind 7");
    assert_eq!(E::Pos { expr: 1, r#in: 2 }.to_string(), "POSITION(1 IN 2)");
}
