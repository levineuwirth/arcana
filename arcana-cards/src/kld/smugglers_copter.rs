//! Smuggler's Copter — `{2}` Artifact — Vehicle, 3/3, Flying, Crew 1.
//! "Whenever Smuggler's Copter attacks or blocks, you may draw a card. If you
//!  do, discard a card." — WIRED as two loot triggers (SelfAttacks + SelfBlocks:
//!  draw 1, then discard 1). The "you may" is modeled as always looting (the
//!  optional skip is the accepted loot-fidelity gap; cf. loch_dragon). A Vehicle
//!  is not a creature until crewed; Crew 1 animates it into an artifact creature
//!  EOT.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smuggler's Copter");
    let vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vehicle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(crew_ability(1))
            .with_triggered_ability(loot_trigger(1, TriggerCondition::SelfAttacks))
            .with_triggered_ability(loot_trigger(2, TriggerCondition::SelfBlocks)),
    )
}

/// "Whenever this attacks/blocks, loot (draw 1, then discard 1)."
fn loot_trigger(id: u32, cond: TriggerCondition) -> TriggeredAbilityDef {
    TriggeredAbilityDef {
        id,
        trigger_condition: cond,
        intervening_if: None,
        effect: loot,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }
}

fn loot(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}

/// "Crew N: this Vehicle becomes an artifact creature until end of turn."
fn crew_ability(n: u32) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: format!("Crew {n}."),
        cost: ActivationCost { crew: Some(n), ..Default::default() },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: true,
        face_gate: None,
        effect: crew,
    }
}

fn crew(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::add_type(
            ctx.source,
            ctx.source,
            TypeLine::CREATURE.into(),
            Duration::EndOfTurn,
        ),
    }]
}
