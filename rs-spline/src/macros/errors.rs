/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[allow(unused_macros)]
macro_rules! error_kind {
    ($(rename_all: $lit:literal,)? $vis:vis enum $name:ident $($rest:tt)*) => {
        error_kind!(@impl $(rename_all: $lit,)? $vis enum $name $($rest)*);
    };
    (@impl $vis:vis enum $name:ident $($rest:tt)*) => {
        error_kind!(@impl rename_all: "PascalCase", $vis enum $name $($rest)*);

    };
    (@impl rename_all: $lit:literal, $vis:vis enum $name:ident $($rest:tt)*) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIs,
            strum::VariantNames,
        )]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = $lit)
        )]
        #[strum(serialize_all = $lit)]
        $vis enum $name $($rest)*
    };
}
