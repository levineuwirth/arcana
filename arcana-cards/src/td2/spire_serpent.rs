//! Spire Serpent — `{4}{U}` 3/5 Creature — Serpent (U).
//! Defender. Metalcraft — as long as you control three or more
//! artifacts, this creature gets +2/+2 and can attack as though it
//! didn't have defender.
//!
//! Defender is an evergreen keyword carried as a base characteristic.
//! GAP: the Metalcraft static (conditional +2/+2 and the conditional
//! removal of defender for attacking) is a pure continuous static
//! ability with no expressible triggered/activated wiring here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spire Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: Metalcraft static — "as long as you control three or more
    // artifacts, this creature gets +2/+2 and can attack as though it
    // didn't have defender" is a conditional continuous static, not a
    // triggered/activated ability expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
