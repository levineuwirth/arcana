//! Higure, the Still Wind — `{3}{U}{U}` 3/4 Legendary Human Ninja.
//! "Ninjutsu {2}{U}{U}" (deferred — not an expressible keyword).
//! "Whenever Higure deals combat damage to a player, you may search
//! your library for a Ninja card, reveal it, put it into your hand,
//! then shuffle."
//! "{2}: Target Ninja creature can't be blocked this turn."
//!
//! Ninjutsu is not in the usable keyword surface, so keywords are empty
//! and the ninjutsu line is GAP'd. The combat-damage tutor and the
//! {2}-make-unblockable activated ability are both wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::layers::Duration;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Higure, the Still Wind");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Ninjutsu {2}{U}{U}" — Ninjutsu is not an expressible keyword.

    // Build the Ninja filter before the mutable borrow of `reg.register`.
    let ninja_target_filter = script::subtype_filter(reg, "Ninja");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: tutor_ninja,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Target Ninja creature can't be blocked this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ninja_target_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_unblockable,
            }),
    )
}

fn tutor_ninja(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: script::subtype_filter(reg, "Ninja"),
        reveal: true,
    }]
}

fn make_unblockable(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::CantBeBlocked {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}
