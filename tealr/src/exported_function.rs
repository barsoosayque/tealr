use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    string::FromUtf8Error,
};

use crate::{type_generator::NameContainer, TealType};

#[cfg(any(feature = "rlua", feature = "mlua"))]
use crate::{Direction, TealMultiValue};
#[cfg(any(feature = "rlua", feature = "mlua"))]
fn get_all_generics(children: impl Iterator<Item = TealType>) -> HashSet<TealType> {
    let mut generics = HashSet::new();
    for teal_type in children {
        let child_generics =
            get_all_generics(teal_type.generics.clone().unwrap_or_default().into_iter());
        generics.extend(child_generics);
        if teal_type.type_kind.is_generic() {
            generics.insert(teal_type);
        }
    }
    generics
}

///Contains the data needed to write down the type of a function
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExportedFunction {
    ///Name of the function
    pub name: NameContainer,
    ///Any generic type parameters that this function may have
    pub generics: HashSet<TealType>,
    ///The parameters of this function
    pub params: Vec<TealType>,
    ///What this function returns
    pub returns: Vec<TealType>,
    ///If this function is a meta_method
    pub is_meta_method: bool,
}
impl ExportedFunction {
    ///Creates an ExportedFunction with the given name, Parameters and return value
    ///```no_run
    ///# use tealr::ExportedFunction;
    ///ExportedFunction::new::<(String,String),String>(b"concat".to_vec(),false);
    ///```
    #[cfg(any(feature = "rlua", feature = "mlua"))]
    pub fn new<Params: TealMultiValue, Response: TealMultiValue>(
        name: Vec<u8>,
        is_meta_method: bool,
    ) -> Self {
        let params = Params::get_types(Direction::FromLua);
        let returns = Response::get_types(Direction::ToLua);
        let generics = get_all_generics(
            params
                .clone()
                .into_iter()
                .chain(returns.clone().into_iter()),
        );
        Self {
            name: name.into(),
            params,
            returns,
            is_meta_method,
            generics,
        }
    }
    pub(crate) fn generate(
        self,
        self_type: Option<Cow<'static, str>>,
        documentation: &HashMap<NameContainer, String>,
    ) -> std::result::Result<String, FromUtf8Error> {
        let params = self_type
            .iter()
            .map(|v| v.to_owned())
            .chain(self.params.iter().map(|v| v.name.to_owned()))
            .collect::<Vec<_>>()
            .join(", ");

        let returns = self
            .returns
            .iter()
            .map(|v| v.name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");
        let documentation = match documentation.get(&self.name) {
            None => "".to_string(),
            Some(x) => x.lines().map(|v| format!("--{}\n", v)).collect(),
        };

        let name = String::from_utf8(self.name.0)?;

        Ok(format!(
            "{}{}{}: function{}({}):({})",
            documentation,
            if self.is_meta_method {
                "metamethod "
            } else {
                ""
            },
            name,
            if self.generics.is_empty() {
                "".to_owned()
            } else {
                format!("<{}>", {
                    let mut x = self
                        .generics
                        .into_iter()
                        .map(|v| v.name)
                        .collect::<Vec<_>>();
                    x.sort();
                    x.join(",")
                })
            },
            params,
            returns
        ))
    }
}
