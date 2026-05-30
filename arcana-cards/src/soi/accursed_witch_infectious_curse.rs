//! Accursed Witch // Infectious Curse — `{3}{B}` Human Shaman creature 4/2.
//! Spells your opponents cast that target this creature cost {1} less to cast.
//! When this creature dies, return it to the battlefield transformed under
//! your control attached to target opponent.
//! Back face "Infectious Curse": Enchantment — Aura Curse.
//! Enchant player.
//! Spells you cast that target enchanted player cost {1} less to cast.
//! At the beginning of enchanted player's upkeep, that player loses 1 life
//! and you gain 1 life.
//!
//! # GAP
//! 1. "Spells opponents cast that target this creature cost {1} less" —
//!    static cost-reduction effect; no Effect/trigger models this.
//! 2. "When this creature dies, return it transformed attached to target
//!    opponent" — dies trigger with Transform + Attach to a player is not
//!    expressible (Attach targets a permanent, not a player; Aura Curse
//!    enchants a player). GAP'd.
//! 3. Back-face Aura Curse static/triggered abilities not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Accursed Witch");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "spells opponents cast targeting this creature cost {1} less"
        // static cost-reduction not modeled
        ..Default::default()
    };

    // GAP: "when this creature dies, return it transformed attached to target
    // opponent" — ZoneChange dies trigger + Transform + Attach-to-player not
    // expressible.

    let back_name = reg.interner_mut().intern("Infectious Curse");
    let aura_sub = reg.interner_mut().intern("Aura");
    let curse_sub = reg.interner_mut().intern("Curse");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);
    back_subtypes.0.insert(curse_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            // GAP: back-face static "spells targeting enchanted player cost
            // {1} less" and upkeep trigger "lose 1 life / gain 1 life" not
            // modeled (back-face-only triggered ability not modeled).
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
