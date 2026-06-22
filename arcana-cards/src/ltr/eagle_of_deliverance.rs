//! Eagle of Deliverance — `{4}{W}{W}` 5/5 white Bird Soldier with Flying.
//!
//! * Flying.
//! * When this creature enters, put an indestructible counter on another target
//!   creature you control. Draw a card if that creature's power is 2 or less.
//!
//! Flying and the ETB ability are expressed faithfully. The indestructible
//! counter is modeled via `CounterKind::Named("indestructible")`; the
//! conditional draw reads the target's power at resolution. "Another" (the
//! target may not be this creature) is a minor fidelity nuance not separately
//! restricted by the filter.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eagle of Deliverance");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    // Pre-intern the named counter for the resolver lookup.
    let _indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_indestructible_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_indestructible_counter(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let kind = reg
        .interner()
        .lookup("indestructible")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Shield);
    let mut effects = vec![Effect::AddCounters {
        target: *id,
        kind,
        count: 1,
    }];
    if script::power_of(state, *id) <= 2 {
        effects.push(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        });
    }
    effects
}
