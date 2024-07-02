// src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SerialSensors, attributes(sensor))]
pub fn derive_serial_sensors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("#[derive(RuntimeTypeInformation)] can only be used with enums");
    };

    let sensor_match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let sensor_type = quote! { <crate::types::#variant_name as ::serial_sensors_proto_traits::CompileTimeTypeInformation>::SENSOR };
        quote! {
            #name::#variant_name(_) => #sensor_type,
        }
    });

    let field_match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let field_type = quote! { <crate::types::#variant_name as ::serial_sensors_proto_traits::CompileTimeTypeInformation>::FIELD };
        quote! {
            #name::#variant_name(_) => #field_type,
        }
    });

    let num_components_match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let num_components = quote! { <crate::types::#variant_name as ::serial_sensors_proto_traits::CompileTimeTypeInformation>::NUM_COMPONENTS };
        quote! {
            #name::#variant_name(_) => #num_components,
        }
    });

    let expanded = quote! {
        impl ::serial_sensors_proto_traits::RuntimeTypeInformation for #name {
            fn sensor(&self) -> ::serial_sensors_proto_traits::SensorType {
                match self {
                    #( #sensor_match_arms )*
                }
            }

            fn field(&self) -> ::serial_sensors_proto_traits::ValueType {
                match self {
                    #( #field_match_arms )*
                }
            }

            fn num_components(&self) -> u8 {
                match self {
                    #( #num_components_match_arms )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
