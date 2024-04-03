use proc_macro2::TokenStream;

pub struct ClientModDef {
    /// The name of the module.
    pub name: String,
    /// The name of the client defined in this module.
    pub client_name: String,
    /// The tokens that provide the required definitions for the client.
    pub client_tokens: TokenStream,
    /// Children modules.
    pub children: Vec<ClientModDef>,
}
