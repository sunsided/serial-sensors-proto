extern crate proc_macro;

use darling::ast::Fields;
use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_macro_input, DeriveInput, Field, Path, Type};

#[derive(Debug, FromMeta)]
struct SensorAttributes {
    id: u8,
    data: Path,
    components: u8,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(sensor))]
struct Version1DataVariant {
    ident: syn::Ident,
    fields: Fields<Field>,
    #[darling(flatten)]
    sensor: SensorAttributes,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(sensor), supports(enum_newtype))]
struct Version1Data {
    ident: syn::Ident,
    data: darling::ast::Data<Version1DataVariant, darling::util::Ignored>,
}

#[proc_macro_derive(SerialSensors, attributes(sensor))]
pub fn derive_runtime_type_information(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let version1_data = Version1Data::from_derive_input(&input).expect("Failed to parse input");

    let name = &version1_data.ident;

    let mut sensor_match_arms = Vec::new();
    let mut field_match_arms = Vec::new();
    let mut num_components_match_arms = Vec::new();
    let mut from_impls = Vec::new();
    let mut encode_match_arms = Vec::new();
    let mut decode_match_arms = Vec::new();

    let mut sensor_types = HashSet::new();
    let mut duplicate_error = None;

    if let darling::ast::Data::Enum(variants) = &version1_data.data {
        for variant in variants {
            let variant_name = &variant.ident;
            let sensor_type = &variant.sensor.id;
            let field_type = &variant.sensor.data;
            let num_components = variant.sensor.components;

            let variant_name_str = variant_name.to_string();

            if !sensor_types.insert(sensor_type) {
                duplicate_error = Some(quote! {
                    compile_error!(concat!("Duplicate sensor type found (", #sensor_type, ") at ", #variant_name_str));
                });
                break;
            }

            // Extract the type of the variant's field
            let variant_field_type = &variant.fields.fields[0].ty;

            from_impls.push(quote! {
                impl crate::CompileTimeTypeInformation for #variant_field_type {
                    const TYPE_ID: u8 = #sensor_type;
                    const VALUE_TYPE: crate::ValueType = #field_type;
                    const NUM_COMPONENTS: u8 = #num_components;
                }

                impl core::convert::From< #variant_field_type > for #name {
                    fn from(value: #variant_field_type) -> #name {
                        #name :: #variant_name ( value )
                    }
                }

                impl TryFrom< #name > for #variant_field_type {
                    type Error = ();

                    fn try_from(value: #name) -> Result<#variant_field_type, Self::Error> {
                        match value {
                            #name :: #variant_name (value) => Ok(value),
                            _ => Err(())
                        }
                    }
                }
            });

            sensor_match_arms.push(quote! {
                #name::#variant_name(_) => #sensor_type,
            });

            field_match_arms.push(quote! {
                #name::#variant_name(_) => #field_type,
            });

            num_components_match_arms.push(quote! {
                #name::#variant_name(_) => #num_components,
            });

            encode_match_arms.push(quote! {
                #name::#variant_name(value) => ::bincode::Encode::encode(&value, encoder)?,
            });

            decode_match_arms.push(quote! {
                (#sensor_type, #field_type) => {
                    let value: #variant_field_type = ::bincode::Decode::decode(decoder)?;
                    Ok(#name :: #variant_name ( value ))
                }
            });
        }
    }

    let expanded = if let Some(error) = duplicate_error {
        error
    } else {
        quote! {
            impl #name {
                pub const fn sensor_type_id(&self) -> u8 {
                    match self {
                        #( #sensor_match_arms )*
                    }
                }

                pub const fn value_type(&self) -> crate::ValueType {
                    match self {
                        #( #field_match_arms )*
                    }
                }

                pub const fn num_components(&self) -> u8 {
                    match self {
                        #( #num_components_match_arms )*
                    }
                }
            }

            impl crate::RuntimeTypeInformation for #name {
                fn sensor_type_id(&self) -> u8 {
                    match self {
                        #( #sensor_match_arms )*
                    }
                }

                fn value_type(&self) -> crate::ValueType {
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

            #( #from_impls )*

            impl ::bincode::Encode for #name {
                fn encode<__E: ::bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), ::bincode::error::EncodeError> {
                    use crate::RuntimeTypeInformation;
                    bincode::Encode::encode(&self.sensor_type_id(), encoder)?;
                    bincode::Encode::encode(&(self.value_type() as u8), encoder)?;
                    // don't encode the component count; sensor ID and type are enough
                    match self {
                        #( #encode_match_arms )*
                    }
                    Ok(())
                }
            }

            impl ::bincode::Decode for #name {
                fn decode<__D: bincode::de::Decoder>(
                    decoder: &mut __D,
                ) -> Result<Self, ::bincode::error::DecodeError> {
                    let type_id: u8 = bincode::Decode::decode(decoder)?;
                    let value_type: u8 = bincode::Decode::decode(decoder)?;
                    let value_type = crate::ValueType::try_from(value_type).map_err(|_| ::bincode::error::DecodeError::Other("An unknown combination of type ID and value type was detected"))?;
                    match (type_id, value_type) {
                        #( #decode_match_arms )*,
                        _ => Err(::bincode::error::DecodeError::Other("An unknown combination of type ID and value type was detected"))
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(sensor), supports(struct_newtype))]
struct SensorDataType {
    ident: syn::Ident,
    data: darling::ast::Data<darling::util::Ignored, Type>,
}

#[proc_macro_derive(SensorDataType)]
pub fn derive_sensor_data_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = SensorDataType::from_derive_input(&input).expect("Failed to parse input");

    let name = &data.ident;
    let expanded = if let darling::ast::Data::Struct(fields) = &data.data {
        let field = &fields.fields[0];
        quote! {
            impl #name {
                /// Constructs a new instance of the [`#name`] type.
                pub const fn new(value: #field) -> Self {
                    Self(value)
                }
            }

            impl core::convert::AsRef<#field> for #name {
                fn as_ref(&self) -> &#field {
                    &self.0
                }
            }

            impl core::convert::AsMut<#field> for #name {
                fn as_mut(&mut self) -> &mut #field {
                    &mut self.0
                }
            }

            impl core::ops::Deref for #name {
                type Target = #field;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl core::ops::DerefMut for #name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl From<#name> for #field {
                fn from(value: #name) -> #field {
                    value.0
                }
            }

            impl TryFrom<crate::versions::Version1DataFrame> for #name {
                type Error = ();

                #[inline]
                fn try_from(value: crate::versions::Version1DataFrame) -> Result<Self, Self::Error> {
                    value.value.try_into()
                }
            }

            impl TryFrom<crate::VersionedDataFrame<crate::versions::Version1, crate::versions::Version1DataFrame>> for #name {
                type Error = ();

                #[inline]
                fn try_from(value: crate::VersionedDataFrame<crate::versions::Version1, crate::versions::Version1DataFrame>) -> Result<Self, Self::Error> {
                    value.data.value.try_into()
                }
            }
        }
    } else {
        quote! {}
    };

    TokenStream::from(expanded)
}
