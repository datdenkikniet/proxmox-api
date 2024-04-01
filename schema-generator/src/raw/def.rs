use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Lit;

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub rename: Option<String>,
    pub name: String,
    pub ty: TokenStream,
    pub optional: bool,
    pub primitive_ty: Option<PrimitiveTypeDef>,
}

impl ToTokens for FieldDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldDef {
            rename,
            name,
            ty,
            optional: _,
            primitive_ty,
        } = self;

        let name = Ident::new(name, quote!().span());

        let rename = if let Some(rename) = rename {
            let renamed = Lit::new(Literal::string(&rename));
            Some(quote!(#[serde(rename = #renamed)]))
        } else {
            None
        };

        tokens.extend(quote! {
            #rename
            pub #name: #ty,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimitiveTypeDef {
    String,
    Number,
    Integer,
    Boolean,
}

impl ToTokens for PrimitiveTypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let this_tokens = match self {
            PrimitiveTypeDef::String => quote!(String),
            PrimitiveTypeDef::Number => quote!(f32),
            PrimitiveTypeDef::Integer => quote!(u32),
            PrimitiveTypeDef::Boolean => quote!(bool),
        };

        tokens.extend(this_tokens)
    }
}

#[derive(Clone)]
pub enum TypeDef {
    Unit,
    Primitive(PrimitiveTypeDef),
    Array {
        inner: Box<TypeDef>,
    },
    Struct {
        name: TokenStream,
        derives: Vec<String>,
        fields: Vec<FieldDef>,
        external_defs: Vec<TypeDef>,
    },
}

impl TypeDef {
    pub const DEFAULT_DERIVES: [&'static str; 4] = [
        "Clone",
        "Debug",
        "::serde::Serialize",
        "::serde::Deserialize",
    ];

    pub fn primitive(&self) -> Option<PrimitiveTypeDef> {
        if let Self::Primitive(p) = self {
            Some(*p)
        } else {
            None
        }
    }

    pub fn new_struct<'a, I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: TokenStream,
        extra_derives: T,
        fields: Vec<FieldDef>,
        external_defs: Vec<TypeDef>,
    ) -> Self {
        Self::Struct {
            name,
            derives: Self::DEFAULT_DERIVES
                .into_iter()
                .map(str::to_string)
                .chain(extra_derives.into_iter().map(|e| e.as_ref().to_string()))
                .map(|v| v.to_string())
                .collect(),
            fields,
            external_defs,
        }
    }

    pub fn as_field_ty(&self, optional: bool) -> TokenStream {
        let ty = match self {
            TypeDef::Unit => quote!(()),
            TypeDef::Struct { name, .. } => name.clone(),
            TypeDef::Primitive(name) => name.to_token_stream(),
            TypeDef::Array { inner } => {
                let inner = inner.as_field_ty(optional);
                quote!(Vec<#inner>)
            }
        };

        if optional {
            quote!(Option<#ty>)
        } else {
            ty
        }
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDef::Primitive(_) | TypeDef::Unit => {}
            TypeDef::Array { inner } => inner.to_tokens(tokens),
            TypeDef::Struct {
                name,
                derives,
                fields,
                external_defs,
            } => {
                tokens.extend(external_defs.iter().map(|def| quote! { #def }));

                let derives = derives.iter().map(|v| {
                    let parsed: TokenStream = v.parse().unwrap();
                    quote! { #parsed, }
                });

                let name = &name;

                let fields = &fields;

                tokens.extend(quote! {
                    #[derive(#(#derives)*)]
                    pub struct #name {
                        #(#fields)*
                    }
                });
            }
        }
    }
}
