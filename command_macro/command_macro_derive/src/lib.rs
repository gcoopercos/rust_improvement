extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(CommandMacro)]
pub fn command_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_command_macro(&ast)
}

fn impl_command_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl CommandMacro for #name {
            fn box_eq(&self, other: &dyn Any) -> bool{
                other.downcast_ref::<Self>().map_or(false, |a| self == a)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn command_time(&self) -> u128 {
                self.command_time
            }
        }
    };
    gen.into()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
