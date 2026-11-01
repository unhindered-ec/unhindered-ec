use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{LitStr, Token};

#[derive(Debug, Clone)]
pub enum Import {
    Path(Option<Token![::]>, syn::UseTree),
    SuperRelativePath(syn::UseTree),
}

impl From<syn::UseTree> for Import {
    fn from(value: syn::UseTree) -> Self {
        Self::Path(None, value)
    }
}

impl ToTokens for Import {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Path(leading_colon, path) => {
                let import_statement = format!(
                    "# use {leading_colon}{path};",
                    path = path.to_token_stream().to_string().replace('\n', ""),
                    leading_colon = leading_colon.map_or_default(ToTokens::into_token_stream)
                );
                let import_lit = syn::LitStr::new(&import_statement, Span::mixed_site());
                tokens.extend(quote! {
                    #[doc = #import_lit]
                });
            }
            Self::SuperRelativePath(path) => {
                let import_statement = format!(
                    "::{path};",
                    path = path.to_token_stream().to_string().replace('\n', "")
                );
                let import_lit = syn::LitStr::new(&import_statement, Span::mixed_site());
                tokens.extend(quote! {
                    #[doc = ::std::concat!("# use ::" , ::std::module_path!() , #import_lit)]
                });
            }
        }
    }
}

pub fn doctest<'a, 'b>(
    imports: Option<impl IntoIterator<Item = &'a Import>>,
    intro: Option<&str>,
    tokens: &str,
    outro: Option<&str>,
    attribute: Option<&'b str>,
) -> TokenStream {
    let mut token_stream = TokenStream::new();
    if let Some(attr) = attribute {
        let doc_line = format!(" ```{attr}");
        let lit = LitStr::new(&doc_line, Span::mixed_site());
        token_stream.extend(quote! {
            #[doc = #lit]
        });
    } else {
        token_stream.extend(quote! {
            /// ```
        });
    }
    for import in imports.into_iter().flatten() {
        import.to_tokens(&mut token_stream);
    }

    if let Some(intro) = intro {
        let _ = create_doc_attrs_from_string(intro, &mut token_stream, false);
    }
    let _ = create_doc_attrs_from_string(tokens, &mut token_stream, true);

    if let Some(outro) = outro {
        let _ = create_doc_attrs_from_string(outro, &mut token_stream, false);
    }
    token_stream.extend(quote! {
        /// ```
    });

    token_stream
}

fn create_doc_attrs_from_string(
    input: &str,
    tokens: &mut TokenStream,
    do_format: bool,
) -> Result<(), syn::Error> {
    let line_iter = if do_format {
        let input_block = format!("fn main() {{ {input}  }}");
        let file_contents = vec![syn::Item::Fn(syn::parse_str(&input_block)?)];
        let formatted = prettyplease::unparse(&syn::File {
            frontmatter: None,
            shebang: None,
            attrs: vec![],
            items: file_contents,
        });
        let mut without_lines: Vec<String> = formatted
            .lines()
            .skip(1)
            .map(|l| format!(" {line}", line = l.get(4..).unwrap_or_default()))
            .collect();

        // remove the last item, if any
        without_lines.pop();

        without_lines
    } else {
        input
            .lines()
            .map(|line| format!(" {line}"))
            .collect::<Vec<String>>()
    };
    for line in line_iter {
        let line_lit = LitStr::new(&line, Span::mixed_site());
        tokens.extend(quote! {
            #[doc = #line_lit]
        });
    }
    Ok(())
}
