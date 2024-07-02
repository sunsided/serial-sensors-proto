// src/lib.rs
extern crate proc_macro;

use darling::{FromDeriveInput, FromField, FromMeta, FromMetaItem, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[derive(Debug, FromMeta)]
struct SensorAttributes {
    sensor_type: String,
    data: String,
    components: u8,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(sensor))]
struct Version1DataVariant {
    ident: syn::Ident,
    #[darling(flatten)]
    sensor: SensorAttributes,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(sensor), supports(enum_any))]
struct Version1Data {
    ident: syn::Ident,
    data: darling::ast::Data<Version1DataVariant, darling::util::Ignored>,
}

#[proc_macro_derive(SerialSensors, attributes(sensor))]
pub fn derive_runtime_type_information(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let version1_data = Version1Data::from_derive_input(&input).expect("Failed to parse input");

    let name = &version1_data.ident;

    let mut sensor_match_arms: Vec<String> = Vec::new();
    let mut field_match_arms: Vec<String> = Vec::new();
    let mut num_components_match_arms: Vec<u8> = Vec::new();

    if let darling::ast::Data::Enum(variants) = &version1_data.data {
        for variant in variants {
            let variant_name = &variant.ident;
            let sensor_type = &variant.sensor.sensor_type;
            let field_type = &variant.sensor.data;
            let num_components = variant.sensor.components;

            /*
            sensor_match_arms.push(quote! {
                #name::#variant_name(_) => crate::types::#sensor_type::SENSOR,
            });
            field_match_arms.push(quote! {
                #name::#variant_name(_) => crate::types::#field_type::FIELD,
            });
            num_components_match_arms.push(quote! {
                #name::#variant_name(_) => crate::types::#field_type::NUM_COMPONENTS,
            });
             */
        }
    }

    let expanded = quote! {
        impl ::serial_sensors_proto_traits::RuntimeTypeInformation for #name {
            fn sensor(&self) -> ::serial_sensors_proto_traits::SensorType {
                match self {
                    _ => todo!(),
                    #( #sensor_match_arms )*
                }
            }

            fn field(&self) -> ::serial_sensors_proto_traits::ValueType {
                match self {
                    _ => todo!(),
                    #( #field_match_arms )*
                }
            }

            fn num_components(&self) -> u8 {
                match self {
                    _ => todo!(),
                    #( #num_components_match_arms )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
