//! Ephara's Enlightenment — `{1}{W}{U}` enchantment — Aura (Theros Beyond Death).
//! "Enchant creature. When this Aura enters, put a +1/+1 counter on
//!  enchanted creature. Enchanted creature has flying. Whenever a
//!  creature you control enters, you may return this Aura to its owner's
//!  hand."
//!
//! ETB: place a +1/+1 counter on the host (read via `source.attached_to`)
//! and install `attached_keyword(Flying)`. The third clause is a
//! ZoneChange trigger on a creature you control entering the battlefield;
//! its effect returns the Aura to hand. (The "you may" is approximated as
//! a mandatory return — no may-wrapper in the demonstrated API.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ephara's Enlightenment");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_creature_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Flying,
            Duration::WhileSourceOnBattlefield,
        ),
    }];
    if let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::AddCounters {
            target: host,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects
}

fn on_creature_enters(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // "you may return this Aura to its owner's hand" — may approximated as
    // mandatory; no may-wrapper in the demonstrated API.
    vec![Effect::ReturnToHand { target: trig.source }]
}
