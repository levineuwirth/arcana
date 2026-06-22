//! Stormwild Capridor — `{2}{W}` 1/3 Bird Goat with Flying.
//!
//! * Flying (keyword).
//! * "If noncombat damage would be dealt to this creature, prevent
//!   that damage. Put a +1/+1 counter on this creature for each 1
//!   damage prevented this way." GAP: this is a static damage-
//!   prevention REPLACEMENT effect (CR 615) with a counter rider; it
//!   has no trigger word and no activation cost, so it cannot be wired
//!   as a triggered or activated ability with the usable surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormwild Capridor");
    let bird = reg.interner_mut().intern("Bird");
    let goat = reg.interner_mut().intern("Goat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(goat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: static noncombat-damage prevention replacement with +1/+1
        // counter rider — no triggered/activated form in the usable surface.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
