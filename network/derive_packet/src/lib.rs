use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, Type};
use quote::quote;

#[proc_macro_attribute]
pub fn derive_packet(args: TokenStream, tokens: TokenStream) -> TokenStream {

    let input: DeriveInput = syn::parse(tokens).unwrap();
    let name = &input.ident;
    let args = proc_macro2::TokenStream::from(args);

    let id_tokens = quote!{
        const ID: VarInt = VarInt(#args);
    };

    let fields: Vec<(&Option<Ident>, &Type)>;

    match &input.data {
        syn::Data::Struct(data) => {
            fields = data.fields.iter().map(|f| (&f.ident, &f.ty)).collect();
        },
        _ => {
            panic!("Not a struct!");
        }
    }

    let mut read_fields = quote!();
    let mut read_fields_2 = quote!();
    for (f, t) in &fields {
        let f = f.as_ref().unwrap();

        match t {
            syn::Type::Path(tp) => {
                let tp = &tp.path;
                match &tp.segments.first().unwrap().arguments {
                    syn::PathArguments::AngleBracketed(ger) => {
                        match ger.args.first().unwrap() {
                            syn::GenericArgument::Type(_) => {
                                read_fields.extend(quote!{
                                    let #f = Vec::read(r)?;
                                });
                            },
                            _ => {panic!("Invalid generic type provided")}
                        }
                    },
                    syn::PathArguments::None => {
                        read_fields.extend(quote!{
                            let #f = #t::read(r)?;
                        });
                    }
                    _ => {panic!("Invalid type provided!")}
                }

            },
            _ => {
                panic!("Invalid type");
            }
        }

        read_fields_2.extend(quote!(
            #f,
        ));
    }

    let read_tokens = quote!{
        fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
            #read_fields

            Ok(Self {
                #read_fields_2
            })
        }
    };

    let mut write_fields = quote!();
    for (f, _) in fields {
        let f = f.as_ref().unwrap();
        write_fields.extend(quote!(
            self.#f.write(&mut out)?;
        ));
    }

    let write_tokens = quote!{
        fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
            let mut out: Vec<u8> = Vec::new();
            Self::ID.write(&mut out)?;

            #write_fields
            
            VarInt(out.len() as i32).write(w)?;
            w.write(&out)?;
            Ok(())
        }
    };

    TokenStream::from(quote! {
        #input
        
        impl Packet for #name {
            #id_tokens
            #read_tokens
            #write_tokens
        }

    })
}