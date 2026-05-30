//! Sorin of House Markov // Sorin, Ravenous Neonate — `{1}{B}` Legendary Human
//! Noble 1/4. Front face: Lifelink. Extort. At the beginning of each of your
//! postcombat main phases, if you gained 3 or more life this turn, exile Sorin
//! then return him to the battlefield transformed.
//! Back face (Sorin, Ravenous Neonate): Legendary Planeswalker — Sorin [4
//! loyalty]. Extort. +2: Create a Food token. -1: Sorin deals damage equal to
//! the amount of life you gained this turn to any target. -6: Gain control of
//! target creature. It becomes a Vampire. Put a lifelink counter on it if you
//! control a white permanent other than that creature or Sorin.
//!
//! GAP: Extort keyword not in engine keyword set.
//! GAP: "If you gained 3 or more life this turn" condition — life-gained-this-turn
//! counter not accessible via script helpers; transform trigger omitted.
//! GAP: Back-face planeswalker loyalty abilities not modeled (no loyalty-ability API).
//! GAP: "-1: Sorin deals damage equal to the amount of life you gained this turn"
//! — life-gained-this-turn dynamic amount not accessible.
//! GAP: "-6: gain control, becomes Vampire, lifelink counter if white permanent"
//! — type-change and conditional counter not expressible.
//! GAP: Back-face-only triggered/activated abilities not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin of House Markov");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(noble_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        // GAP: Extort keyword not in engine keyword set
        // GAP: front-face postcombat main phase transform trigger (life >= 3) not modeled
        ..Default::default()
    };

    // Back face: Sorin, Ravenous Neonate — Legendary Planeswalker — Sorin, loyalty 4
    let back_name = reg.interner_mut().intern("Sorin, Ravenous Neonate");
    let sorin_sub = reg.interner_mut().intern("Sorin");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sorin_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(4),
            // GAP: Extort keyword not in engine keyword set
            // GAP: planeswalker loyalty abilities not modeled
            // GAP: back-face-only triggered ability not modeled
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
