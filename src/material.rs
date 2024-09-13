use enum_map::{enum_map, Enum, EnumMap};

use crate::tag::Tag;

#[derive(Clone, Copy, Debug, Enum)]
pub enum Material {
    Bronze,
    Fire,
    Glass,
    Iron,
    Metal,
    Steel,
    Stone,
    Wood,
}

pub struct MaterialData {
    pub parent: Option<Material>,
    pub tags: Vec<Tag>
}

pub type MaterialMap = EnumMap<Material, MaterialData>;

pub fn generate_material_map() -> MaterialMap {
    let result: MaterialMap = enum_map! {
        Material::Bronze => MaterialData{
            parent: Some(Material::Metal),
            tags: vec!()
        },
        Material::Fire => MaterialData{
            parent: None,
            tags: vec!(Tag::Burning, Tag::Gas, Tag::Transparent)
        },
        Material::Glass => MaterialData{
            parent: None,
            tags: vec!(Tag::Solid, Tag::Transparent)
        },
        Material::Iron => MaterialData{
            parent: Some(Material::Metal),
            tags: vec!()
        },
        Material::Metal => MaterialData{
            parent: None,
            tags: vec!(Tag::Conductive, Tag::Solid)
        },
        Material::Steel => MaterialData{
            parent: Some(Material::Metal),
            tags: vec!()
        },
        Material::Stone => MaterialData{
            parent: None,
            tags: vec!(Tag::Solid)
        },
        Material::Wood => MaterialData{
            parent: None,
            tags: vec!(Tag::Organic, Tag::Solid, Tag::Flammable)
        },
    };

    return result;
}
