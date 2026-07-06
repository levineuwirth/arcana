//! Caldera Pyremaw — `{3}{R}{R}` 3/3 Dragon with Flying.
//! "Whenever you cast an instant or sorcery spell, put a +1/+1 counter on
//!  this creature. Then this creature deals damage equal to its power to
//!  target opponent."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caldera Pyremaw");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                ),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: grow_and_burn,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // NOTE: "target opponent" approximated as target player; no
            // opponent-only player target helper in the demonstrated surface.
            target_requirements: vec![TargetRequirement::target_opponent()],
        }),
    )
}

fn grow_and_burn(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Power after the just-added +1/+1 counter (current power + 1).
    let amount = (script::power_of(state, trig.source) + 1).max(0) as u32;
    let mut out = vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if let Some(TargetChoice::Player(p)) = trig.targets.targets.first() {
        out.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(*p),
            amount,
        });
    }
    out
}
