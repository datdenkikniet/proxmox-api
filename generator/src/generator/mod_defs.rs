//! This module contains data structures related to managing
//! similarly-named definitions within a module.

use std::collections::BTreeMap;

use quote::ToTokens;

use super::{
    BoundedIntegerDef, BoundedNumberDef, BoundedStringDef, EnumDef, NumItemsDef, StructDef, TypeDef,
};

#[derive(Debug, Clone, Default)]
pub struct ModuleDefs {
    enums: BTreeMap<String, EnumDef>,
    structs: BTreeMap<String, StructDef>,
    num_items: BTreeMap<String, NumItemsDef>,
    bounded_integers: BTreeMap<String, BoundedIntegerDef>,
    bounded_numbers: BTreeMap<String, BoundedNumberDef>,
    bounded_strings: BTreeMap<String, BoundedStringDef>,
}

impl Extend<TypeDef> for ModuleDefs {
    fn extend<T: IntoIterator<Item = TypeDef>>(&mut self, iter: T) {
        iter.into_iter().for_each(|v| self.push(v))
    }
}

impl ModuleDefs {
    pub fn push(&mut self, def: TypeDef) {
        match def {
            TypeDef::Struct(strt) => {
                if let Some(prev_strt) = self.structs.get(&strt.name()) {
                    if prev_strt != &strt {
                        panic!("Encountered structs on the same level with exactly the same name that are not equal!\n{strt:#?}\n{prev_strt:#?}");
                    }
                } else {
                    self.structs.insert(strt.name().to_string(), strt);
                }
            }
            TypeDef::Enum(mut en) => {
                let original_name = en.name();
                let mut name = original_name.clone();
                let mut counter = 2;
                let mut found_duplicate = false;

                while let Some(previous_def) = self.enums.get(&name) {
                    if previous_def == &en {
                        found_duplicate = true;
                        break;
                    } else {
                        name = format!("{original_name}{counter}");
                        counter += 1;
                    }
                }

                if !found_duplicate {
                    en.set_name(&name);
                    self.enums.insert(en.name(), en);
                }
            }
            TypeDef::NumberedItems(mut new_items) => {
                let original_name = new_items.name();
                let mut name = original_name.clone();
                let mut counter = 2;
                let mut found_duplicate = false;

                while let Some(value) = self.num_items.get(&name) {
                    if value == new_items.as_ref() {
                        found_duplicate = true;
                        break;
                    } else {
                        name = format!("{original_name}{counter}");
                        counter += 1;
                    }
                }
                if !found_duplicate {
                    new_items.set_name(&name);
                    self.num_items.insert(name, *new_items);
                }
            }
            TypeDef::Array(_) | TypeDef::Primitive(_) | TypeDef::KnownType { .. } => {}
            TypeDef::BoundedInteger(def) => {
                if let Some(prev_def) = self.bounded_integers.get(&def.name) {
                    if prev_def != &def {
                        panic!(
                            "Encountered bounded integers with the same name that are not equal!\n{def:#?}\n{prev_def:#?}"
                        );
                    }
                } else {
                    self.bounded_integers
                        .insert(def.name.to_string(), def.clone());
                }
            }
            TypeDef::BoundedNumber(def) => {
                if let Some(prev_def) = self.bounded_numbers.get(&def.name) {
                    if prev_def != &def {
                        panic!(
                            "Encountered bounded numbers with the same name that are not equal!\n{def:#?}\n{prev_def:#?}"
                        );
                    }
                } else {
                    self.bounded_numbers
                        .insert(def.name.to_string(), def.clone());
                }
            }
            TypeDef::BoundedString(def) => {
                if let Some(prev_def) = self.bounded_strings.get(&def.name) {
                    if prev_def != &def {
                        panic!(
                            "Encountered bounded strings with the same name that are not equal!\n{def:#?}\n{prev_def:#?}"
                        );
                    }
                } else {
                    self.bounded_strings
                        .insert(def.name.to_string(), def.clone());
                }
            }
        }
    }
}

impl ToTokens for ModuleDefs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let structs = self.structs.values();
        let enums = self.enums.values();
        let items = self.num_items.values();
        let bounded_integers = self.bounded_integers.values();
        let bounded_numbers = self.bounded_numbers.values();
        let bounded_strings = self.bounded_strings.values();

        tokens.extend(quote::quote! {
            #(#structs)*

            #(#enums)*

            #(#items)*

            #(#bounded_integers)*

            #(#bounded_numbers)*

            #(#bounded_strings)*
        });
    }
}
