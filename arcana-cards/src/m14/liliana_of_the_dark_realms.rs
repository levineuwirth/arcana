//! Liliana of the Dark Realms — `{2}{B}{B}` Legendary Planeswalker — Liliana, starting loyalty 3.
//!
//! +1: Search your library for a Swamp card, reveal it, put it into your
//!   hand, then shuffle. Modeled with `Effect::Search` (subtype Swamp,
//!   destination hand, reveal).
//! −3: Target creature gets +X/+X or -X/-X until end of turn, where X is
//!   the number of Swamps you control. GAP: dynamic-X (Swamp count) pump
//!   isn't expressible; the ability shell carries the correct −3 cost.
//! −6: You get an emblem with "Swamps you control have '{T}: Add
//!   {B}{B}{B}{B}.'" The emblem grants an activated mana ability to a
//!   filtered land set — no anthem/keyword/filtered builder expresses
//!   it, so the emblem is created but its grant is GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana of the Dark Realms");
    let liliana = reg.interner_mut().intern("Liliana");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _emblem = reg.interner_mut().intern("Liliana of the Dark Realms emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Search your library for a Swamp card, reveal it, \
                       put it into your hand, then shuffle.".into(),
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
                effect: plus_one_tutor,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target creature gets +X/+X or -X/-X until end of \
                       turn, where X is the number of Swamps you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Swamps you control have \
                       '{T}: Add {B}{B}{B}{B}.'\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn plus_one_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg.interner().lookup("Swamp").expect("Swamp interned at register");
    vec![Effect::Search {
        player: ctx.controller,
        zone: Zone::Library(ctx.controller),
        filter: ObjectFilter {
            subtypes: Some(vec![swamp]),
            ..Default::default()
        },
        destination: Zone::Hand(ctx.controller),
        reveal: true,
    }]
}

fn minus_three_pump(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic-X pump (X = number of Swamps you control) with a
    // +/- choice isn't expressible.
    Vec::new()
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Liliana of the Dark Realms emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: granting an activated mana ability ("{T}: Add {B}{B}{B}{B}")
            // to Swamps you control isn't expressible via the static
            // builders (anthem/keyword/filtered pump).
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
