//! Blood Hypnotist — `{2}{R}` 3/3 Vampire.
//! "This creature can't block."
//! "Whenever you sacrifice one or more Blood tokens, target creature can't block
//! this turn. This ability triggers only once each turn."
//!
//! The sacrifice-Blood trigger is implemented: a Sacrificed trigger filtered to
//! Blood subtype, OncePerTurn, that forbids a target creature from blocking this
//! turn. GAP: the static "This creature can't block" is a continuous restriction
//! with no expressible primitive.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blood Hypnotist");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let blood_filter = script::subtype_filter(reg, "Blood");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "This creature can't block" — continuous static restriction, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Sacrificed {
                filter: blood_filter,
            },
            intervening_if: None,
            effect: forbid_target_block,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn forbid_target_block(
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
    vec![Effect::ForbidBlocking {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}
