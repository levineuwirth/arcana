//! Narset Transcendent — `{2}{W}{U}` Legendary Planeswalker — Narset, starting loyalty 6.
//! +1: Look at top card; if noncreature/nonland, may reveal + put in hand — GAP (no look-at-top primitive).
//! -2: Next instant/sorcery you cast gains rebound — GAP (rebound rider not buildable).
//! -9: Emblem "Your opponents can't cast noncreature spells" — emitted as CreateEmblem with empty
//!     statics/abilities (rule-altering restriction not buildable by anthem/keyword/filtered).

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narset Transcendent");
    let sub = reg.interner_mut().intern("Narset");
    let _emblem = reg.interner_mut().intern("Narset Transcendent emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top card of your library. If it's a \
                       noncreature, nonland card, you may reveal it and put it \
                       into your hand."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_look,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: When you next cast an instant or sorcery spell from \
                       your hand this turn, it gains rebound."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_rebound,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: You get an emblem with \"Your opponents can't cast \
                       noncreature spells.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_emblem,
            }),
    )
}

/// `+1` — look at top, may reveal a noncreature/nonland.
fn plus_one_look(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no look-at-top-then-may-reveal primitive in the demonstrated surface.
    Vec::new()
}

/// `-2` — next instant/sorcery gains rebound.
fn minus_two_rebound(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: rebound rider on a future cast is not buildable from the Effect surface.
    Vec::new()
}

/// `-9` — emblem: opponents can't cast noncreature spells.
fn minus_nine_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Narset Transcendent emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: rule-altering "your opponents can't cast noncreature spells"
            // is not expressible via anthem/keyword/filtered statics; emit the
            // emblem shell so the catalog records it.
            statics: vec![],
            abilities: vec![],
        },
    }]
}
