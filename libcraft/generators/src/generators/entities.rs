use convert_case::{Case, Casing};

use crate::utils::*;

pub fn generate() {
    let entities: Vec<EntityInfo> = load_minecraft_json("entities.json").unwrap();
    let mut out = generate_enum!(
        EntityKind,
        entities.iter  ()
            .map(|e| e.name.to_case(Case::UpperCamel))
            .collect::<Vec<_>>(),
        [serde::Serialize, serde::Deserialize, bincode::Encode, bincode::Decode],
        #[serde(try_from = "String", into = "&'static str")]
    );

    out.extend(generate_enum_property!(
        EntityKind,
        "id",
        u32,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let id = e.id;
                quote! { #id }
            }))
            .collect(),
        true
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "width",
        f32,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let width = e.width;
                quote! { #width }
            }))
            .collect()
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "height",
        f32,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let height = e.height;
                quote! { #height }
            }))
            .collect()
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "name",
        &str,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let name = &e.name;
                quote! { #name }
            }))
            .collect(),
        true,
        &'static str
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "namespaced_id",
        &str,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let namespaced_id = format!("minecraft:{}", e.name);
                quote! { #namespaced_id }
            }))
            .collect(),
        true,
        &'static str
    ));

    out.extend(quote! {
        use std::convert::TryFrom;
        use std::str::FromStr;

        impl TryFrom<String> for EntityKind {
            type Error = &'static str;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if let Some(kind) = EntityKind::from_name(value.as_str()) {
                    Ok(kind)
                } else {
                    Err("Unknown entity kind")
                }
            }
        }

        impl From<EntityKind    > for &'static str {
            fn from(i: EntityKind) -> Self {
                i.name()
            }
        }

        impl FromStr for EntityKind {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some(kind) = EntityKind::from_name(s) {
                    Ok(kind)
                } else {
                    Err("Unknown entity kind")
                }
            }
        }
    });

    output("libcraft/core/src/entity.rs", out.to_string().as_str());

    let mut markers = quote! {
        use vane::Component;
    };
    for entity in entities.iter() {
        let name = format_ident!("{}", entity.name.to_case(Case::UpperCamel));
        let doc = format!("A marker component for {} entities.", entity.name);
        markers.extend(quote! {
            #[derive(Debug, Copy, Clone)]
            #[doc = #doc]
            pub struct #name;
            impl Component for #name {}
        });
    }
    output("quill/src/entities.rs", markers.to_string().as_str());

    for entity in entities.iter() {
        let path = &format!("feather/common/src/entities/{}.rs", entity.name);
        let file = std::fs::read_to_string(path);
        if file.is_err() || file.unwrap().starts_with(GENERATED_COMMENT) {
            let name = format_ident!("{}", entity.name.to_case(Case::UpperCamel));
            output(
                path,
                quote! {
                    use vane::EntityBuilder;
                    use quill::entities::#name;
                    use quill::components::EntityKindComponent;
                    use libcraft::EntityKind;

                    pub fn build_default(builder: &mut EntityBuilder) {
                        super::build_default(builder);
                        builder.add(#name).add(EntityKindComponent(EntityKind::#name));
                    }
                }
                .to_string()
                .as_str(),
            );
        }
    }

    let name_snake = entities
        .iter()
        .map(|e| format_ident!("{}", e.name))
        .collect::<Vec<_>>();
    let name_upper_camel = entities
        .iter()
        .map(|e| format_ident!("{}", e.name.to_case(Case::UpperCamel)));
    output(
        "feather/common/src/entities.rs",
        quote! {
            use libcraft::EntityKind;
            use vane::EntityBuilder;
            use quill::components::{OnGround, EntityUuid};
            use uuid::Uuid;

            #[doc = "Adds default components shared between all entities."]
            fn build_default(builder: &mut EntityBuilder) {
                builder
                    .add(EntityUuid(Uuid::new_v4()))
                    .add(OnGround(true));
            }

            #(pub mod #name_snake;)*

            pub fn add_entity_components(builder: &mut EntityBuilder, kind: EntityKind) {
                match kind {
                    #(EntityKind::#name_upper_camel => #name_snake::build_default(builder)),*,
                }
            }
        }
        .to_string()
        .as_str(),
    );
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EntityInfo {
    id: u32,
    name: String,
    width: f32,
    height: f32,
}
