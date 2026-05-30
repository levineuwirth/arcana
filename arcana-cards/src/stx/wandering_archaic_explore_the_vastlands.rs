//! Wandering Archaic // Explore the Vastlands
//! Front: {5} Creature — Avatar 4/4
//! Whenever an opponent casts an instant or sorcery spell, they may pay {2}.
//! If they don't, you may copy that spell. You may choose new targets for the copy.
//!
//! Back: "Explore the Vastlands" — Sorcery (no mana cost on back per MDFC rules for sorcery back).
//! Each player looks at the top five cards of their library and may reveal a land card
//! and/or an instant or sorcery card from among them. Each player puts the cards they
//! revealed this way into their hand and the rest on the bottom of their library in a
//! random order. Each player gains 3 life.
//!
//! GAP: Front face trigger "whenever an opponent casts an instant or sorcery spell, they may pay
//!      {2}. If they don't, you may copy that spell" — the CastSpell trigger condition is not
//!      available in the engine (no TriggerCondition::SpellCast by opponent); CopySpell also
//!      requires the stack-object id of the spell, which is not accessible from a triggered
//!      ability at that moment.
//! GAP: Back face "look at top 5, may reveal a land AND/OR an instant/sorcery" —
//!      DigTopN only takes one card; two-category simultaneous picks are not modeled.
//!      The effect is approximated as two DigTopN calls (each GAP'd below) but the
//!      "same 5 cards" constraint can't be modeled; we emit Vec::new() for the whole back effect.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wandering Archaic");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: triggered ability "whenever an opponent casts an instant/sorcery, they may pay {2}.
        //       If they don't, you may copy that spell." — TriggerCondition::SpellCast not available.
        ..Default::default()
    };

    // Back face: Explore the Vastlands — Sorcery
    let back_name = reg.interner_mut().intern("Explore the Vastlands");
    let back_chars = Characteristics {
        name: back_name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    let back_ability = SpellAbilityDef {
        text: "Each player looks at the top five cards of their library and may reveal a land card and/or an instant or sorcery card from among them. Each player puts the cards they revealed this way into their hand and the rest on the bottom of their library in a random order. Each player gains 3 life.".into(),
        target_requirements: vec![],
        modal: None,
        effect: explore_the_vastlands_resolve,
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: Some(back_ability),
    };

    reg.register(CardDefinition::new(name, chars).with_mdfc_back(back_face))
}

fn explore_the_vastlands_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player looks at top 5, may reveal a land and/or instant/sorcery card" —
    //      DigTopN only models a single-category take from top N; the two-category simultaneous
    //      pick from the same 5 cards is not expressible. GainLife per player also requires
    //      iterating players, but the whole effect is GAP'd due to the pick constraint.
    Vec::new()
}
