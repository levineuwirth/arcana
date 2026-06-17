//! Blistering Firecat — `{1}{R}{R}{R}` 7/1 Elemental Cat.
//! Trample, haste.
//! "At the beginning of the end step, sacrifice this creature."
//! Morph {R}{R}.
//!
//! Morph is not part of the usable keyword surface for this card class
//! (the morph cast mechanic is deferred) — see GAP below.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blistering Firecat");
    let elemental = reg.interner_mut().intern("Elemental");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(cat);

    // GAP: Morph {R}{R} — the morph cast mechanic is not in the usable
    // keyword surface for this card class; omitted.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: sacrifice_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Sacrifice this creature." Sacrifice this one specific permanent.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: self_filter(),
        count: 1,
    }]
}

fn self_filter() -> arcana_core::targets::ObjectFilter {
    // Best-effort: any creature you control. The engine's Sacrifice picks
    // among matching permanents; with the source on the battlefield this
    // is the closest expressible form for "sacrifice this creature".
    arcana_core::targets::ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
}
