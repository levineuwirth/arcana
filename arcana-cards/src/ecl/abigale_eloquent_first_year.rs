//! Abigale, Eloquent First-Year — `{W/B}{W/B}` 1/1 Legendary Bird Bard.
//! Flying, first strike, lifelink.
//! When Abigale enters, up to one other target creature loses all
//! abilities. Put a flying counter, a first strike counter, and a
//! lifelink counter on that creature.
//!
//! Keyword line → keywords vec. The ETB is a single targeted trigger
//! (up to one target creature): strip its abilities, then place the
//! three keyword counters (modeled as Named counters). "other" is not
//! expressible in ObjectFilter (no source-exclusion predicate) — the
//! filter matches any creature, a minor fidelity note.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abigale, Eloquent First-Year");
    let bird = reg.interner_mut().intern("Bird");
    let bard = reg.interner_mut().intern("Bard");
    // Pre-intern the keyword-counter names so the resolver's read-only
    // lookup resolves them.
    let _ = reg.interner_mut().intern("flying");
    let _ = reg.interner_mut().intern("first strike");
    let _ = reg.interner_mut().intern("lifelink");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(bard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W/B}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_strip_and_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_strip_and_grant(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let flying = reg.interner().lookup("flying").map(CounterKind::Named);
    let fs = reg.interner().lookup("first strike").map(CounterKind::Named);
    let ll = reg.interner().lookup("lifelink").map(CounterKind::Named);
    let mut out = vec![Effect::LoseAllAbilities {
        target: *id,
        duration: Duration::Permanent,
    }];
    if let Some(k) = flying {
        out.push(Effect::AddCounters { target: *id, kind: k, count: 1 });
    }
    if let Some(k) = fs {
        out.push(Effect::AddCounters { target: *id, kind: k, count: 1 });
    }
    if let Some(k) = ll {
        out.push(Effect::AddCounters { target: *id, kind: k, count: 1 });
    }
    out
}
