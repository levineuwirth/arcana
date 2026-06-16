//! Gleam of Authority — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1 for each +1/+1
//!  counter on other creatures you control. Enchanted creature has
//!  vigilance and '{W}, {T}: Bolster 1.'"
//!
//! Partial: the dynamic +1/+1 buff is an `attached_pt_dynamic` whose
//! compute fn sums the +1/+1 counters on OTHER creatures the Aura
//! controller controls (excluding the host) — a counter-count, not a
//! named-subtype count, so it is expressible. Vigilance is the canonical
//! `attached_keyword` install. GAP'd: the granted "{W}, {T}: Bolster 1"
//! activated ability (Bolster is not an expressible effect here).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gleam of Authority");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted "{W}, {T}: Bolster 1" activated ability — Bolster is
    // not an expressible effect on a granted host ability here.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt_dynamic(
                trig.source,
                counters_pump,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Vigilance,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn counters_pump(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(src) = state.object_or_lki(source) else {
        return (0, 0);
    };
    let you = src.controller;
    let host = src.attached_to;
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let mut total: u32 = 0;
    for id in script::ids_matching(state, &filter, you) {
        if Some(id) == host {
            continue; // "other creatures you control"
        }
        if let Some(o) = state.objects.get(id) {
            total += o.count_counters(CounterKind::PlusOnePlusOne);
        }
    }
    let n = total as i32;
    (n, n)
}
