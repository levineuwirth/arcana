//! Walker of Secret Ways — `{2}{U}` 1/2 blue Human Ninja.
//!
//! Ninjutsu {1}{U}
//! Whenever this creature deals combat damage to a player, look at that player's
//! hand.
//! {1}{U}: Return target Ninja you control to its owner's hand. Activate only
//! during your turn.
//!
//! GAP: Ninjutsu is not an expressible KeywordAbility (the alternate-cost return
//! mechanic is engine debt) — keywords left empty.
//! GAP: the combat-damage trigger's only effect is "look at that player's hand",
//! a hidden-information peek with no Effect primitive — the trigger fires but its
//! resolver is empty.
//! The "{1}{U}: Return target Ninja you control to hand" activation is wired,
//! gated to your turn via `activation_condition`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::conditions;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Walker of Secret Ways");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let self_name = reg.interner().lookup("Walker of Secret Ways");
    let ninja_filter = script::subtype_filter(reg, "Ninja").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ninjutsu {1}{U} — no KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter {
                        name: self_name,
                        ..ObjectFilter::default()
                    },
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: peek_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}: Return target Ninja you control to its owner's hand. Activate only during your turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    activation_condition: Some(only_your_turn),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ninja_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bounce_ninja,
            }),
    )
}

fn peek_hand(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at that player's hand" — hidden-information peek, no Effect.
    Vec::new()
}

fn only_your_turn(state: &GameState, source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::is_your_turn(state, source, you)
}

fn bounce_ninja(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}
