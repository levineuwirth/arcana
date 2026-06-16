//! PuPu UFO — `{2}` 0/4 Artifact Creature — Construct Alien with Flying.
//! {T}: You may put a land card from your hand onto the battlefield.
//! {3}: Until end of turn, this creature's base power becomes equal to the
//! number of Towns you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("PuPu UFO");
    let construct = reg.interner_mut().intern("Construct");
    let alien = reg.interner_mut().intern("Alien");
    let _town = reg.interner_mut().intern("Town");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    subtypes.0.insert(alien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: You may put a land card from your hand onto the battlefield.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_land_from_hand,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}: Until end of turn, this creature's base power becomes equal to the number of Towns you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: set_base_power_to_towns,
            }),
    )
}

fn put_land_from_hand(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: arcana_core::targets::ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into()),
        tapped: false,
    }]
}

fn set_base_power_to_towns(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Town")
        .controlled_by(arcana_core::targets::ControllerConstraint::You);
    let n = script::count_matching(state, &filter, ctx.controller) as i32;
    // Oracle changes only base POWER; SetBasePT must set toughness too — keep
    // the printed toughness of 4 (documented fidelity approximation).
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: n,
        toughness: 4,
        duration: Duration::EndOfTurn,
    }]
}
