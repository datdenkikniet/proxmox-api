use proc_macro2::TokenStream;

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub definition: TokenStream,
}
