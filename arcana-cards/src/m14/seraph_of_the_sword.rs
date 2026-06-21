//! Seraph of the Sword — `{3}{W}` 3/3 Angel.
//! Flying.
//! Prevent all combat damage that would be dealt to this creature.
//!
//! The keyword line (Flying) is a base characteristic. The combat-
//! damage-prevention clause is a STATIC replacement effect on the
//! creature itself (CR 614); there is no triggered/activated form, and
//! `Effect::PreventDamageFrom` cannot express "to THIS creature only,
//! combat damage only" — so that ability is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seraph of the Sword");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static replacement "Prevent all combat damage that would be
    // dealt to this creature" — no Effect can express a self-scoped,
    // combat-only prevention shield as a continuous static ability.
    reg.register(CardDefinition::new(name, chars))
}
