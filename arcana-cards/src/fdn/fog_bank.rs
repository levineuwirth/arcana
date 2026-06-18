//! Fog Bank — `{1}{U}` 0/2 blue Wall with Flying and Defender.
//! "Prevent all combat damage that would be dealt to and dealt by this creature."
//!
//! Flying + Defender are base keywords. The damage-prevention clause is a pure
//! static replacement effect with no trigger or activation cost to hang an
//! `Effect` on, so it is GAP'd here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fog Bank");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static replacement "prevent all combat damage dealt to and by this
    // creature" — no trigger/activation to attach an Effect to; would need a
    // self-installed PreventDamageFrom static on the source.
    reg.register(CardDefinition::new(name, chars))
}
