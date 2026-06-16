//! Kaya the Inexorable — `{3}{W}{B}` Legendary Planeswalker — Kaya, starting loyalty 5.
//! +1: Put a ghostform counter on up to one target nontoken creature; it gains a leaves-the-battle
//!     return ability. IMPLEMENTED — the named "ghostform" counter is added; the conferred
//!     triggered ability is GAP'd (not expressible here).
//! −3: Exile target nonland permanent. IMPLEMENTED.
//! −7: You get an emblem with "At your upkeep, cast a legendary spell from hand/graveyard/exile
//!     for free." Emblem emitted; cast-from-anywhere-free not buildable so the emblem's ability is
//!     GAP'd (empty).

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya the Inexorable");
    let sub = reg.interner_mut().intern("Kaya");
    let _ghostform = reg.interner_mut().intern("ghostform");
    let _emblem = reg.interner_mut().intern("Kaya the Inexorable emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put a ghostform counter on up to one target nontoken creature. It \
                       gains \"When this creature dies or is put into exile, return it to its \
                       owner's hand and create a 1/1 white Spirit creature token with flying.\""
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::CREATURE.into())
                            .nontoken(),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_ghostform,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Exile target nonland permanent.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_exile,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"At the beginning of your upkeep, you may cast \
                       a legendary spell from your hand, from your graveyard, or from among cards \
                       you own in exile without paying its mana cost.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1` — add a ghostform counter to up to one nontoken creature (granted ability GAP'd).
fn plus_one_ghostform(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Up to one target" — if no target was chosen, nothing happens.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let ghostform = reg.interner().lookup("ghostform").expect("ghostform interned");
    // GAP: the conferred triggered ability ("When this dies or is exiled, return it + make a
    // Spirit") is not expressible here; only the named counter is added.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(ghostform),
        count: 1,
    }]
}

/// `−3` — exile target nonland permanent.
fn minus_three_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}

/// `−7` — emblem (cast-from-anywhere-free not buildable; emit emblem shell with GAP'd ability).
fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Kaya the Inexorable emblem")
        .expect("emblem name interned");
    // GAP: "cast a legendary spell from hand/graveyard/exile without paying its mana cost" is not
    // expressible; emit the emblem with empty abilities.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![],
        },
    }]
}
