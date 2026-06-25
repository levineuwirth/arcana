//! Nettling Imp — `{2}{B}` 1/1 black Imp.
//! `{T}: Choose target non-Wall creature the active player has
//! controlled continuously since the beginning of the turn. That
//! creature attacks this turn if able. Destroy it at the beginning of
//! the next end step if it didn't attack this turn. Activate only
//! during an opponent's turn, before attackers are declared.`
//!
//! The "attacks this turn if able" clause is wired (a must_attack requirement
//! on the chosen creature, EndOfTurn). GAP: "destroy it if it didn't attack",
//! "activate only during an opponent's turn before attackers are declared",
//! and the non-Wall / active-player-controlled-continuously target restriction
//! are not expressible with the current Effect/ActivationCost API.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nettling Imp");
    let imp = reg.interner_mut().intern("Imp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(imp);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
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
                text: "{T}: Choose target non-Wall creature the active player has controlled continuously since the beginning of the turn. That creature attacks this turn if able. Destroy it at the beginning of the next end step if it didn't attack this turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: force_attack,
            }),
    )
}

fn force_attack(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // "That creature attacks this turn if able." GAP: "destroy it if it
    // didn't attack", the opponent's-turn-before-attackers timing, and the
    // non-Wall / active-player-controlled-continuously target restriction are
    // not in the Effect/ActivationCost catalog.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(ctx.source, *id, Duration::EndOfTurn),
    }]
}
