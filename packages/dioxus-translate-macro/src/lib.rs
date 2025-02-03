extern crate proc_macro;

use std::cell::RefCell;
use std::rc::Rc;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_macro_input, DeriveInput, Ident, Lit, LitStr, Meta, Token};

#[proc_macro]
pub fn translate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TranslateInput);

    let struct_name = input.struct_name;
    let mut fields = Vec::new();
    let mut ko_impl = Vec::new();
    let mut en_impl = Vec::new();

    for field in input.fields {
        let field_name = field.field_name;

        fields.push(quote! {
            pub #field_name: &'static str,
        });

        let mut ko_value = None;
        let mut en_value = None;

        for translation in field.translations {
            if translation.lang == "ko" {
                ko_value = Some(translation.value);
            } else if translation.lang == "en" {
                en_value = Some(translation.value);
            }
        }

        ko_impl.push(quote! {
            #field_name: #ko_value,
        });

        en_impl.push(quote! {
            #field_name: #en_value,
        });
    }

    let en = quote! {
            fn en() -> Self {
                Self {
                    #(#en_impl)*
                }
            }
    };

    #[allow(unused_variables)]
    let ko = quote! {};

    #[cfg(feature = "ko")]
    let ko = quote! {
        fn ko() -> Self {
                Self {
                    #(#ko_impl)*
                }
            }

    };

    let expanded = quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #struct_name {
            #(#fields)*
        }

        impl dioxus_translate::Translator for #struct_name {
            #en

            #ko
        }
    };

    TokenStream::from(expanded)
}

/// Macro input structure
struct TranslateInput {
    struct_name: Ident,
    fields: Vec<FieldTranslations>,
}

struct FieldTranslations {
    field_name: Ident,
    translations: Vec<LanguageTranslation>,
}

struct LanguageTranslation {
    lang: Ident,
    value: String,
}

impl Parse for TranslateInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the struct name
        let struct_name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        let mut fields = Vec::new();

        // Parse fields
        while !input.is_empty() {
            let field_name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let content;
            braced!(content in input);

            let mut translations = Vec::new();

            while !content.is_empty() {
                let lang: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let value: Lit = content.parse()?;
                content.parse::<Token![,]>().ok(); // Allow trailing commas
                if let Lit::Str(lit_str) = value {
                    translations.push(LanguageTranslation {
                        lang,
                        value: lit_str.value(),
                    });
                }
            }

            fields.push(FieldTranslations {
                field_name,
                translations,
            });

            input.parse::<Token![,]>().ok(); // Allow trailing commas
        }

        Ok(TranslateInput {
            struct_name,
            fields,
        })
    }
}

/// Implements a custom derive macro for `Translate`, which automatically generates
/// a `translate(&self, lang: &Language) -> &'static str` method for enums.
///
/// This macro extracts `#[translate(ko = "...")]` attributes from the enum variants
/// and maps them to Korean translations. If no translation is provided, the variant
/// name is used as the default English translation.
#[proc_macro_derive(Translate, attributes(translate))]
pub fn translate_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = ast.ident;

    // Ensure that the derive macro is applied to an enum
    let variants = match ast.data {
        syn::Data::Enum(ref data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(enum_name, "Translate can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let mut en_arms = Vec::new();
    let mut ko_arms = Vec::new();
    let mut display_arms = Vec::new();
    let mut from_str_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;
        let default_str = variant_ident.to_string();
        let en_translation = Rc::new(RefCell::new(default_str.clone()));
        let ko_translation = Rc::new(RefCell::new(default_str.clone()));

        // Process attributes to extract translations
        for attr in &variant.attrs {
            if let Meta::List(ref meta_list) = attr.meta {
                if meta_list.path.is_ident("translate") {
                    let en = Rc::clone(&en_translation);
                    let ko = Rc::clone(&ko_translation);

                    let _ = meta_list.parse_nested_meta(move |nv| {
                        if nv.path.is_ident("en") {
                            let s: LitStr = nv.value()?.parse()?;
                            *en.borrow_mut() = s.value();
                        } else if nv.path.is_ident("ko") {
                            let s: LitStr = nv.value()?.parse()?;
                            *ko.borrow_mut() = s.value();
                        }

                        Ok(())
                    });
                }
            }
        }

        let en_str = syn::LitStr::new(&en_translation.borrow(), proc_macro2::Span::call_site());
        let ko_str = syn::LitStr::new(&ko_translation.borrow(), proc_macro2::Span::call_site());
        let lower_name = syn::LitStr::new(
            &variant_ident.to_string().to_lowercase(),
            proc_macro2::Span::call_site(),
        );

        en_arms.push(quote! {
            #enum_name::#variant_ident => #en_str,
        });
        ko_arms.push(quote! {
            #enum_name::#variant_ident => #ko_str,
        });
        display_arms.push(quote! {
            #enum_name::#variant_ident => write!(f, #lower_name),
        });

        from_str_arms.push(quote! {
            #en_str | #ko_str | #lower_name => Ok(#enum_name::#variant_ident),
        });
    }

    // Generate the implementation block for `translate`
    let gen = quote! {
        impl #enum_name {
            pub fn translate(&self, lang: &dioxus_translate::Language) -> &'static str {
                match lang {
                    dioxus_translate::Language::En => match self {
                        #(#en_arms)*
                    },
                    dioxus_translate::Language::Ko => match self {
                        #(#ko_arms)*
                    },
                }
            }
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }


        impl std::str::FromStr for ProjectArea {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    #(#from_str_arms)*
                    _ => Err(format!("invalid field")),
                }
            }
        }
    };

    gen.into()
}

#[cfg(test)]
mod translate_tests {
    use super::*;
    use dioxus_translate_types::Language;
    use std::str::FromStr;

    #[derive(Translate)]
    pub enum ProjectArea {
        #[translate(ko = "경제")]
        Economy = 1,
        #[translate(ko = "사회")]
        Society = 2,
        #[translate(ko = "기술")]
        Technology = 8,
    }

    #[test]
    fn test_translation() {
        assert_eq!(ProjectArea::Economy.translate(&Language::En), "Economy");
        assert_eq!(ProjectArea::Economy.translate(&Language::Ko), "경제");

        assert_eq!(ProjectArea::Society.translate(&Language::En), "Society");
        assert_eq!(ProjectArea::Society.translate(&Language::Ko), "사회");

        assert_eq!(
            ProjectArea::Technology.translate(&Language::En),
            "Technology"
        );
        assert_eq!(ProjectArea::Technology.translate(&Language::Ko), "기술");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ProjectArea::Economy), "economy");
        assert_eq!(format!("{}", ProjectArea::Society), "society");
        assert_eq!(format!("{}", ProjectArea::Technology), "technology");
    }

    #[test]
    fn test_from_str() {
        // Test English names
        assert_eq!(ProjectArea::from_str("Economy"), Ok(ProjectArea::Economy));
        assert_eq!(ProjectArea::from_str("Society"), Ok(ProjectArea::Society));
        assert_eq!(
            ProjectArea::from_str("Technology"),
            Ok(ProjectArea::Technology)
        );

        // Test Korean translations
        assert_eq!(ProjectArea::from_str("경제"), Ok(ProjectArea::Economy));
        assert_eq!(ProjectArea::from_str("사회"), Ok(ProjectArea::Society));
        assert_eq!(ProjectArea::from_str("기술"), Ok(ProjectArea::Technology));

        // Test lowercase names
        assert_eq!(ProjectArea::from_str("economy"), Ok(ProjectArea::Economy));
        assert_eq!(ProjectArea::from_str("society"), Ok(ProjectArea::Society));
        assert_eq!(
            ProjectArea::from_str("technology"),
            Ok(ProjectArea::Technology)
        );

        // Test invalid input
        assert!(ProjectArea::from_str("invalid_field").is_err());
    }
}
