use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{LitStr, Token};

// pub(crate) fn doctest(doctest: &str, attr: Option<&str>) -> TokenStream {
//     let mut doctest_tokenstream = TokenStream::new();

//     if let Some(attr) = attr {
//         let doc_line = format!(" ```{attr}");
//         let lit = LitStr::new(&doc_line, Span::mixed_site());
//         doctest_tokenstream.extend(quote! {
//             #[doc = #lit]
//         });
//     } else {
//         doctest_tokenstream.extend(quote! {
//             /// ```
//         });
//     }
//     for line in doctest.lines() {
//         let line = format!(" {line}");
//         let line_lit = LitStr::new(&line, Span::mixed_site());
//         doctest_tokenstream.extend(quote! {
//             #[doc = #line_lit]
//         })
//     }
//     doctest_tokenstream.extend(quote! {
//         /// ```
//     });
//     doctest_tokenstream
// }

#[derive(Debug, Clone)]
pub(crate) enum Import {
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
            Import::Path(leading_colon, path) => {
                let import_statement = format!(
                    "# use {leading_colon}{path};",
                    path = path.to_token_stream().to_string().replace('\n', ""),
                    leading_colon = leading_colon
                        .map(|lc| lc.into_token_stream())
                        .unwrap_or_default()
                );
                let import_lit = syn::LitStr::new(&import_statement, Span::mixed_site());
                tokens.extend(quote! {
                    #[doc = #import_lit]
                });
            }
            Import::SuperRelativePath(path) => {
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

pub(crate) fn doctest<'a, 'b>(
    imports: Option<impl IntoIterator<Item = &'a Import>>,
    intro: Option<impl ToString>,
    tokens: impl ToString,
    outro: Option<impl ToString>,
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
        create_doc_attrs_from_string(intro.to_string(), &mut token_stream, false);
    }
    create_doc_attrs_from_string(tokens.to_string(), &mut token_stream, true);

    if let Some(outro) = outro {
        create_doc_attrs_from_string(outro.to_string(), &mut token_stream, false);
    }
    token_stream.extend(quote! {
        /// ```
    });

    token_stream
}

fn create_doc_attrs_from_string(input: String, tokens: &mut TokenStream, do_format: bool) {
    let line_iter = if do_format {
        let input_block = format!("fn main() {{ {input}  }}");
        let file_contents = vec![syn::Item::Fn(syn::parse_str(&input_block).unwrap())];
        let formatted = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: vec![],
            items: file_contents,
        });
        let mut without_lines: Vec<String> = formatted
            .lines()
            .skip(1)
            .map(|l| format!(" {line}", line = &l.get(4..).unwrap_or_default()))
            .collect();
        if !without_lines.is_empty() {
            without_lines.swap_remove(without_lines.len() - 1);
        }

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
        })
    }
}
