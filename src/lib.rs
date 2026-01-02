use std::str::FromStr;

use proc_macro::{Delimiter, Group, Ident, Spacing, TokenStream, TokenTree};

/// A procedural macro that splits an optional chaining expression into its segments.
///
/// foo!(test_struct.value?.get((|| 1)())?.value?.value)
///
/// (|| test_struct.value.as_ref()?.get((|| 1)())?.value.as_ref()?.value.as_ref())().copied()
#[proc_macro]
pub fn opt(input: TokenStream) -> TokenStream {
    // dbg!(input.to_string());
    let split_tokens = split_on_optional_chain(input);
    let split_tokens_len = split_tokens.len();
    let as_ref = TokenStream::from_str(".as_ref()?.").unwrap();
    let mut expr = TokenStream::new();
    let mut is_last_optional_chain = false;
    for (i, segment) in split_tokens.into_iter().enumerate() {
        let segment_len = segment.len();
        for (i_tt, tt) in segment.into_iter().enumerate() {
            // Skip the last '?' in the segment
            let is_question_mark = match &tt {
                TokenTree::Punct(p) if p.as_char() == '?' => true,
                _ => false,
            };
            if is_question_mark && i_tt == segment_len - 1 {
                is_last_optional_chain = true;
                // dbg!(segment_len, i_tt, i);
                continue;
            }
            expr.extend(TokenStream::from(tt));
        }
        if i != split_tokens_len - 1 {
            expr.extend(as_ref.clone());
        }
    }
    if is_last_optional_chain {
        expr.extend(TokenStream::from_str(".as_ref()?").unwrap());
    }
    let expr = wrap_some(expr);
    let mut clogure = TokenStream::from_str("|| ").unwrap();
    clogure.extend(expr);
    let resp = call_existing_closure(clogure);
    // dbg!(resp.clone());
    // dbg!(resp.to_string());
    resp
}

fn wrap_some(expr: TokenStream) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend([TokenTree::Ident(Ident::new(
        "Some",
        proc_macro::Span::call_site(),
    ))]);
    ts.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, expr))]);
    ts
}

/// Calls an existing closure represented by the TokenStream.
/// closure: TokenStream ==> represents a closure like `|| expr`
/// responds to TokenStream representing
/// (|| expr )()
fn call_existing_closure(closure: TokenStream) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend([TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        closure,
    ))]);
    ts.extend([TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::new(),
    ))]);
    ts
}

fn split_on_optional_chain(input: TokenStream) -> Vec<Vec<TokenTree>> {
    let mut iter = input.into_iter().peekable();

    let mut segments: Vec<Vec<TokenTree>> = Vec::new();
    let mut current: Vec<TokenTree> = Vec::new();

    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == '?' && p.spacing() == Spacing::Joint => {
                if let Some(TokenTree::Punct(dot)) = iter.peek() {
                    if dot.as_char() == '.' && dot.spacing() == Spacing::Alone {
                        // Finish current segment
                        if !current.is_empty() {
                            segments.push(std::mem::take(&mut current));
                        }

                        // Consume the '.'
                        iter.next();
                        continue;
                    }
                }

                // Not actually '?.' → keep '?'
                current.push(tt);
            }
            _ => current.push(tt),
        }
    }

    if !current.is_empty() {
        segments.push(current);
    }

    segments
}
