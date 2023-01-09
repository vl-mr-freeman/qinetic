//! Label functionality.

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[macro_export]
macro_rules! define_label {

(
        $(#[$label_attr:meta])*
        $label_name:ident,

        $(#[$id_attr:meta])*
        $id_name:ident $(,)?
    ) => {
        $(#[$id_attr])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $id_name(::core::any::TypeId, &'static str);

        impl ::core::fmt::Debug for $id_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "{}", self.1)
            }
        }

        $(#[$label_attr])*
        pub trait $label_name: 'static {
            /// Converts this type into an opaque, strongly-typed label.
            fn as_label(&self) -> $id_name {
                let id = self.type_id();
                let label = self.as_str();
                $id_name(id, label)
            }
            /// Returns the [`TypeId`] used to differentiate labels.
            fn type_id(&self) -> ::core::any::TypeId {
                ::core::any::TypeId::of::<Self>()
            }
            /// Returns the representation of this label as a string literal.
            ///
            /// In cases where you absolutely need a label to be determined at runtime,
            /// you can use [`Box::leak`] to get a `'static` reference.
            fn as_str(&self) -> &'static str;
        }

        impl $label_name for $id_name {
            fn as_label(&self) -> Self {
                *self
            }
            fn type_id(&self) -> ::core::any::TypeId {
                self.0
            }
            fn as_str(&self) -> &'static str {
                self.1
            }
        }

        impl $label_name for &'static str {
            fn as_str(&self) -> Self {
                self
            }
        }
    };
}

pub fn derive_label(
    input: syn::DeriveInput,
    trait_path: &syn::Path,
    attr_name: &str,
) -> TokenStream {
    // return true if the variant specified is an `ignore_fields` attribute
    fn is_ignore(attr: &syn::Attribute, attr_name: &str) -> bool {
        if attr.path.get_ident().as_ref().unwrap() != &attr_name {
            return false;
        }

        syn::custom_keyword!(ignore_fields);
        attr.parse_args_with(|input: syn::parse::ParseStream| {
            let ignore = input.parse::<Option<ignore_fields>>()?.is_some();
            Ok(ignore)
        })
        .unwrap()
    }

    let ident = input.ident.clone();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut where_clause = where_clause.cloned().unwrap_or_else(|| {
        syn::WhereClause {
            where_token: Default::default(),
            predicates: Default::default(),
        }
    });
    where_clause
        .predicates
        .push(syn::parse2(quote! { Self: 'static }).unwrap());

    let as_str = match input.data {
        | syn::Data::Struct(d) => {
            // see if the user tried to ignore fields incorrectly
            if let Some(attr) = d
                .fields
                .iter()
                .flat_map(|f| &f.attrs)
                .find(|a| is_ignore(a, attr_name))
            {
                let err_msg = format!("`#[{attr_name}(ignore_fields)]` cannot be applied to fields individually: add it to the struct declaration");
                return quote_spanned! {
                    attr.span() => compile_error!(#err_msg);
                }
                .into();
            }
            // Structs must either be fieldless, or explicitly ignore the fields.
            let ignore_fields = input.attrs.iter().any(|a| is_ignore(a, attr_name));
            if matches!(d.fields, syn::Fields::Unit) || ignore_fields {
                let lit = ident.to_string();
                quote! { #lit }
            } else {
                let err_msg = format!("Labels cannot contain data, unless explicitly ignored with `#[{attr_name}(ignore_fields)]`");
                return quote_spanned! {
                    d.fields.span() => compile_error!(#err_msg);
                }
                .into();
            }
        },
        | syn::Data::Enum(d) => {
            // check if the user put #[label(ignore_fields)] in the wrong place
            if let Some(attr) = input.attrs.iter().find(|a| is_ignore(a, attr_name)) {
                let err_msg = format!("`#[{attr_name}(ignore_fields)]` can only be applied to enum variants or struct declarations");
                return quote_spanned! {
                    attr.span() => compile_error!(#err_msg);
                }
                .into();
            }
            let arms = d.variants.iter().map(|v| {
                // Variants must either be fieldless, or explicitly ignore the fields.
                let ignore_fields = v.attrs.iter().any(|a| is_ignore(a, attr_name));
                if matches!(v.fields, syn::Fields::Unit) | ignore_fields {
                    let mut path = syn::Path::from(ident.clone());
                    path.segments.push(v.ident.clone().into());
                    let lit = format!("{ident}::{}", v.ident.clone());
                    quote! { #path { .. } => #lit }
                } else {
                    let err_msg = format!("Label variants cannot contain data, unless explicitly ignored with `#[{attr_name}(ignore_fields)]`");
                    quote_spanned! {
                        v.fields.span() => _ => { compile_error!(#err_msg); }
                    }
                }
            });
            quote! {
                match self {
                    #(#arms),*
                }
            }
        },
        | syn::Data::Union(_) => {
            return quote_spanned! {
                input.span() => compile_error!("Unions cannot be used as labels.");
            }
            .into();
        },
    };

    (quote! {
        impl #impl_generics #trait_path for #ident #ty_generics #where_clause {
            fn as_str(&self) -> &'static str {
                #as_str
            }
        }
    })
    .into()
}
