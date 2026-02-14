use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn formats(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let format_str = lit.value();

    let mut fmt_string = String::new();
    let mut expressions: Vec<String> = Vec::new();

    let chars: Vec<char> = format_str.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        match chars[i] {
            '{' if i + 1 < len && chars[i + 1] == '{' => {
                fmt_string.push_str("{{");
                i += 2;
            }
            '{' => {
                i += 1;
                let mut depth = 1u32;
                let mut raw = String::new();
                while i < len && depth > 0 {
                    match chars[i] {
                        '{' => {
                            depth += 1;
                            raw.push('{');
                        }
                        '}' => {
                            depth -= 1;
                            if depth > 0 {
                                raw.push('}');
                            }
                        }
                        c => raw.push(c),
                    }
                    i += 1;
                }

                if depth > 0 {
                    return syn::Error::new(lit.span(), "unclosed `{` in format string")
                        .to_compile_error()
                        .into();
                }

                let raw_chars: Vec<char> = raw.chars().collect();
                let mut spec_start: Option<usize> = None;
                let mut d = 0u32;
                let mut j = 0usize;
                while j < raw_chars.len() {
                    match raw_chars[j] {
                        '(' | '[' | '{' | '<' => d += 1,
                        ')' | ']' | '}' | '>' => d = d.saturating_sub(1),
                        ':' if d == 0 => {
                            if j + 1 < raw_chars.len() && raw_chars[j + 1] == ':' {
                                j += 2;
                                continue;
                            }
                            spec_start = Some(j);
                        }
                        _ => {}
                    }
                    j += 1;
                }

                let (expr, spec) = match spec_start {
                    Some(pos) => {
                        let e: String = raw_chars[..pos].iter().collect();
                        let s: String = raw_chars[pos..].iter().collect();
                        (e, s)
                    }
                    None => (raw, String::new()),
                };

                fmt_string.push('{');
                fmt_string.push_str(&spec);
                fmt_string.push('}');
                expressions.push(expr);
            }
            '}' if i + 1 < len && chars[i + 1] == '}' => {
                fmt_string.push_str("}}");
                i += 2;
            }
            c => {
                fmt_string.push(c);
                i += 1;
            }
        }
    }

    let fmt_literal = fmt_string;
    let expr_tokens: Vec<proc_macro2::TokenStream> = expressions
        .iter()
        .map(|e| {
            e.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|err| panic!("failed to parse expression `{}`: {}", e, err))
        })
        .collect();

    let expanded = quote! {
        format!(#fmt_literal, #(#expr_tokens),*)
    };

    expanded.into()
}
