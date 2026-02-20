use std::{fs, path::Path};

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Error as SynError, Expr, ExprMacro, File as SynFile, LitStr, Macro, visit::Visit};

use crate::{error::LingoraError, rust::RustFile};

#[derive(Debug)]
pub struct MacroCall {
    macro_name: String,
    literal: String,
}

impl MacroCall {
    pub fn literal(&self) -> &str {
        &self.literal
    }
}

impl std::fmt::Display for MacroCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.macro_name, self.literal)
    }
}

pub struct ParsedRustFile {
    file: RustFile,
    syntax: Result<SynFile, SynError>,
    macro_calls: Vec<MacroCall>,
}

impl ParsedRustFile {
    pub fn rust_file(&self) -> &RustFile {
        &self.file
    }

    pub fn path(&self) -> &Path {
        self.file.path()
    }

    pub fn syntax(&self) -> Option<&SynFile> {
        self.syntax.as_ref().ok()
    }

    pub fn macro_calls(&self) -> impl Iterator<Item = &MacroCall> {
        self.macro_calls.iter()
    }

    pub fn error_description(&self) -> String {
        self.syntax
            .as_ref()
            .map_err(|e| e.to_string())
            .err()
            .unwrap_or_default()
    }
}

impl TryFrom<&RustFile> for ParsedRustFile {
    type Error = LingoraError;

    fn try_from(file: &RustFile) -> Result<Self, Self::Error> {
        let file = file.clone();
        let source = fs::read_to_string(file.path())?;
        let syntax = syn::parse_file(&source);

        let mut macro_calls = Vec::new();

        if let Ok(syntax) = &syntax {
            let mut visitor = MacroCallVisitor::default();
            visitor.visit_file(syntax);
            macro_calls.extend(visitor.macro_calls);
        };

        Ok(Self {
            file,
            syntax,
            macro_calls,
        })
    }
}

#[derive(Default)]
struct MacroCallVisitor {
    macro_calls: Vec<MacroCall>,
}

fn record_direct_macro_call(tokens: &TokenStream, macro_name: &str, out: &mut Vec<MacroCall>) {
    if let Some(TokenTree::Literal(literal)) = tokens.clone().into_iter().next()
        && let Ok(literal) = syn::parse2::<LitStr>(literal.into_token_stream())
    {
        let macro_name = String::from(macro_name);
        let literal = literal.value();

        out.push(MacroCall {
            macro_name,
            literal,
        });
    }
}

fn record_literal_macro_calls(tokens: &TokenStream, out: &mut Vec<MacroCall>) {
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            TokenTree::Ident(ident) if matches!(ident.to_string().as_str(), "t" | "te" | "tid") => {
                let macro_name = ident.to_string();

                if let Some(TokenTree::Punct(punct)) = iter.peek()
                    && punct.as_char() == '!'
                {
                    iter.next();

                    if let Some(TokenTree::Group(group)) = iter.next()
                        && group.delimiter() == Delimiter::Parenthesis
                    {
                        record_direct_macro_call(&group.stream(), &macro_name, out);
                    }
                }
            }

            TokenTree::Group(group) => {
                record_literal_macro_calls(&group.stream(), out);
            }

            TokenTree::Literal(literal) => {
                if let Ok(literal) = syn::parse2::<LitStr>(literal.to_token_stream())
                    && let Ok(expr) = syn::parse_str::<Expr>(&literal.value())
                {
                    record_literal_macro_calls(&expr.to_token_stream(), out);
                }
            }

            _ => {}
        }
    }
}

fn handle_macro(mac: &Macro, macro_calls: &mut Vec<MacroCall>) {
    if let Some(ident) = mac.path.get_ident() {
        let macro_name = ident.to_string();

        match macro_name.as_str() {
            "t" | "tid" | "te" => {
                record_direct_macro_call(&mac.tokens, &macro_name, macro_calls);
            }

            _ => record_literal_macro_calls(&mac.tokens, macro_calls),
        }
    }
}

impl<'ast> Visit<'ast> for MacroCallVisitor {
    fn visit_stmt_macro(&mut self, node: &'ast syn::StmtMacro) {
        handle_macro(&node.mac, &mut self.macro_calls);
        syn::visit::visit_stmt_macro(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        handle_macro(&node.mac, &mut self.macro_calls);
        syn::visit::visit_item_macro(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        handle_macro(&node.mac, &mut self.macro_calls);
        syn::visit::visit_expr_macro(self, node);
    }
}
