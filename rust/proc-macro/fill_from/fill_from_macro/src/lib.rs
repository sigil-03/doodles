use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Fill)]
pub fn fill_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_fill(&ast)
}

fn impl_fill(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // let (_impl_generics, type_generics, _where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {
        impl Fill for #name {
            fn generate<String>(input: String) -> #name {
                // println!("{}", #type_generics);
                println!("Generate! {}", stringify!(#name));
                #name::new()
            }
        }
    };
    gen.into()
}
