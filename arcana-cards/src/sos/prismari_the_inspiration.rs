//! Prismari, the Inspiration — `{5}{U}{R}` 7/7 legendary Elder Dragon with Flying.
//! Ward—Pay 5 life (non-mana ward cost — not expressible, GAP'd).
//! "Instant and sorcery spells you cast have storm." (a static that
//!  grants Storm to your spells — not expressible, GAP'd.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prismari, the Inspiration");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        // GAP: Ward—Pay 5 life is a non-mana ward cost (not expressible).
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Instant and sorcery spells you cast have storm" — a static
    // ability-granting effect over your spells, not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
