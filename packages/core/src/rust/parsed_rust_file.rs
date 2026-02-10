use std::{fs, path::Path};

use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Error as SynError, ExprMacro, File as SynFile, LitStr, visit::Visit};

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
            visitor.visit_file(&syntax);
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

fn extract_macro_calls(tokens: TokenStream, out: &mut Vec<MacroCall>) {
    let mut iter = tokens.clone().into_iter().peekable();

    while let Some(tt) = iter.next() {
        if let TokenTree::Ident(ident) = &tt
            && matches!(ident.to_string().as_str(), "t" | "te" | "tid")
        {
            if let Some(TokenTree::Punct(p)) = iter.next()
                && p.as_char() == '!'
                && p.spacing() == Spacing::Alone
            {
                if let Some(TokenTree::Group(group)) = iter.next()
                    && group.delimiter() == Delimiter::Parenthesis
                {
                    if let Some(literal) = group.stream().into_iter().next().and_then(|tt| match tt
                    {
                        TokenTree::Literal(lit) => syn::parse2::<LitStr>(lit.into_token_stream())
                            .ok()
                            .map(|l| l.value()),
                        _ => None,
                    }) {
                        let macro_name = ident.to_string();
                        let macro_call = MacroCall {
                            macro_name,
                            literal,
                        };
                        out.push(macro_call);
                    }
                }
            }
        }
    }

    for tt in tokens {
        if let TokenTree::Group(group) = tt {
            extract_macro_calls(group.stream(), out);
        }
    }
}

impl<'ast> Visit<'ast> for MacroCallVisitor {
    fn visit_stmt_macro(&mut self, i: &'ast syn::StmtMacro) {
        extract_macro_calls(i.mac.tokens.clone(), &mut self.macro_calls);
        syn::visit::visit_stmt_macro(self, i);
    }

    fn visit_item_macro(&mut self, i: &'ast syn::ItemMacro) {
        extract_macro_calls(i.mac.tokens.clone(), &mut self.macro_calls);
        syn::visit::visit_item_macro(self, i);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        let token = node
            .mac
            .tokens
            .clone()
            .into_iter()
            .next()
            .and_then(|tt| match tt {
                TokenTree::Literal(lit) => syn::parse2::<LitStr>(lit.into_token_stream())
                    .ok()
                    .map(|l| l.value()),
                _ => None,
            });

        let ident = node.mac.path.get_ident();

        if let Some(ident) = ident
            && matches!(ident.to_string().as_str(), "rsx")
        {
            let mut macro_calls = Vec::new();
            extract_macro_calls(node.mac.tokens.clone(), &mut macro_calls);
            self.macro_calls.extend(macro_calls);
        } else if let Some(ident) = ident
            && matches!(ident.to_string().as_str(), "t" | "tid" | "te")
            && let Some(literal) = token
        {
            let macro_name = ident.to_string();
            let macro_call = MacroCall {
                macro_name,
                literal,
            };
            self.macro_calls.push(macro_call);
        }

        syn::visit::visit_expr_macro(self, node);
    }
}
