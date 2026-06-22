//! Kappa Tech-Wrecker — `{1}{G}` 1/3 Turtle Ninja.
//!
//! Oracle:
//! * Ninjutsu {1}{G} — alternative-cast keyword, NOT in the usable
//!   `KeywordAbility` surface; GAP'd (no ninjutsu cast primitive).
//! * "This creature enters with a deathtouch counter on it." — modeled as an
//!   ETB trigger that adds a deathtouch counter to itself.
//! * "Whenever this creature deals combat damage to a player, you may remove a
//!   deathtouch counter from it. When you do, exile target artifact or
//!   enchantment that player controls." — wired as a combat-damage trigger that
//!   exiles a target artifact or enchantment an opponent controls.
//!   GAP (partial): the "you may remove a deathtouch counter" reflexive gate
//!   and the strict "that player controls" restriction are not expressible —
//!   best-effort targets an opponent-controlled artifact/enchantment.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kappa Tech-Wrecker");
    let turtle = reg.interner_mut().intern("Turtle");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    subtypes.0.insert(ninja);
    // Intern the deathtouch-counter name so the resolver can recover it.
    let _deathtouch = reg.interner_mut().intern("deathtouch");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Ninjutsu {1}{G} — alternative-cast keyword, no primitive.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_deathtouch_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_deathtouch_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): "enters with a deathtouch counter" is an entering
    // replacement; modeled as an ETB add of a deathtouch counter on itself.
    let Some(kind) = reg.interner().lookup("deathtouch").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}

fn combat_damage_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP (partial): "you may remove a deathtouch counter from it. When you do,"
    // reflexive gate is not expressible — exiling unconditionally.
    vec![Effect::ExilePermanent { target: *id }]
}
