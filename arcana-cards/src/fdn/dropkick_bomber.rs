//! Dropkick Bomber — `{2}{R}` 2/3 Creature — Goblin Warrior.
//!
//! * Other Goblins you control get +1/+1 — a static continuous anthem; GAP'd
//!   (no Effect form for a static lord on a MultiAbilityCreature).
//! * {R}: Until end of turn, another target Goblin you control gains flying and
//!   "When this creature deals combat damage, sacrifice it." — the flying grant
//!   and the granted combat-damage triggered ability are both wired.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dropkick Bomber");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Other Goblins you control get +1/+1" is a static continuous anthem,
    // not a triggered/activated ability — no Effect form here.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}: Until end of turn, another target Goblin you control gains flying and \"When this creature deals combat damage, sacrifice it.\"".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    script::subtype_filter(reg, "Goblin")
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_flying_and_sac_trigger,
        }),
    )
}

fn grant_flying_and_sac_trigger(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "another target Goblin" — the self-exclusion isn't expressible on the
    // target filter (documented fidelity gap); the filter restricts to Goblins.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let granted = TriggeredAbilityDef {
        id: GRANTED_TRIGGER_ID_BASE + 1,
        trigger_condition: TriggerCondition::DamageDealt {
            source_filter: ObjectFilter::creature(),
            target_filter: TargetFilter::AnyTarget,
            combat_only: true,
        },
        intervening_if: None,
        effect: granted_sacrifice_self,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantTriggeredAbility {
            target: *id,
            ability: Box::new(granted),
            duration: Duration::EndOfTurn,
        },
    ]
}

/// Granted "When this creature deals combat damage, sacrifice it."
/// GAP (fidelity): no immediate per-object sacrifice Effect exists, so the
/// sacrifice of the granted creature is scheduled for the next end step via
/// `DelayedAction::Sacrifice` rather than firing immediately.
fn granted_sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Sacrifice,
    }]
}
