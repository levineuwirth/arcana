//! Teferi, Temporal Archmage — `{4}{U}{U}` Legendary Planeswalker — Teferi, starting loyalty 5.
//! +1: Look at top two, put one in hand, other on bottom — GAP (no look-at-top primitive).
//! -1: Untap up to four target permanents — implemented.
//! -10: emblem (rule-altering instant-speed loyalty activation timing) — GAP'd emblem body, emit CreateEmblem.
//! "Teferi, Temporal Archmage can be your commander." — ignored.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Temporal Archmage");
    let sub = reg.interner_mut().intern("Teferi");
    let _emblem = reg.interner_mut().intern("Teferi, Temporal Archmage emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top two cards of your library. Put one of them \
                       into your hand and the other on the bottom of your library."
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
                text: "-1: Untap up to four target permanents.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
                    count: TargetCount::UpTo(4),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: You get an emblem with \"You may activate loyalty abilities of \
                       planeswalkers you control on any player's turn any time you could cast \
                       an instant.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_emblem,
            }),
    )
}

/// `+1`: look at top two, one to hand, other to bottom.
fn plus_one_look(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no look-at-top-then-choose primitive in the demonstrated Effect surface.
    Vec::new()
}

/// `-1`: untap up to four target permanents.
fn minus_one_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in &ctx.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::Untap { target: *id });
        }
    }
    effects
}

/// `-10`: rule-altering timing-permission emblem.
fn minus_ten_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Teferi, Temporal Archmage emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: rule-altering timing permission (activate loyalty abilities at instant
            // speed on any turn) is not expressible via anthem/keyword/filtered builders.
            statics: vec![],
            abilities: vec![],
        },
    }]
}
