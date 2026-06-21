//! Diamond Weapon — `{7}{G}{G}` 8/8 Legendary Artifact Creature — Elemental.
//! "This spell costs {1} less to cast for each permanent card in your
//! graveyard." — a cost-reduction static; not expressible → GAP'd.
//! Reach.
//! "Immune — Prevent all combat damage that would be dealt to Diamond
//! Weapon." — a continuous static prevention with no trigger or cost;
//! not expressible in this card class → GAP'd. (Immune is an ability
//! word, not a keyword variant.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Diamond Weapon");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: "This spell costs {1} less to cast for each permanent card in your
    // graveyard." — dynamic cost reduction; no cost-reduction surface here.
    // GAP: "Immune — Prevent all combat damage that would be dealt to Diamond
    // Weapon." — a continuous static replacement; no triggered/activated hook.
    reg.register(CardDefinition::new(name, chars))
}
