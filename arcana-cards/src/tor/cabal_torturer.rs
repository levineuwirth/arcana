//! Cabal Torturer — `{1}{B}{B}` 1/1 Human Minion.
//! "{B}, {T}: Target creature gets -1/-1 until end of turn."
//! "Threshold — {3}{B}{B}, {T}: Target creature gets -2/-2 until end of
//! turn. Activate only if there are seven or more cards in your graveyard."
//! (Threshold keyword itself is not expressible; the gate is enforced via
//! the activation condition.)

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cabal Torturer");
    let human = reg.interner_mut().intern("Human");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(minion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}: Target creature gets -1/-1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{B}, {T}: Target creature gets -2/-2 until end of turn. Activate only if there are seven or more cards in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").expect("valid cost"),
                    tap: true,
                    activation_condition: Some(seven_in_graveyard),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            }),
    )
}

fn seven_in_graveyard(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::graveyard_at_least(s, you, 7)
}

fn minus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn minus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: -2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
