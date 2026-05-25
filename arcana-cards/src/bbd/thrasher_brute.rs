//! Thrasher Brute — `{3}{B}` 4/3 black Orc Warrior. "Whenever this
//! creature or another Warrior your team controls enters, target
//! opponent loses 1 life and you gain 1 life." Modeled as a generic
//! `ZoneChange` trigger filtered to Warriors entering the battlefield
//! under your control — this naturally catches Thrasher Brute's own
//! ETB (since it's a Warrior) as well as any other Warrior you
//! control. "Target opponent" is encoded via the target requirement's
//! controller constraint.
//!
//! GAP: "your team controls" — Two-Headed Giant teams are not
//! separately modeled by the engine, so the filter falls back to
//! `ControllerConstraint::You`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thrasher Brute");
    let orc = reg.interner_mut().intern("Orc");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let warrior_filter =
        script::subtype_filter(reg, "Warrior").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: warrior_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: on_warrior_etb,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn on_warrior_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: *p,
            amount: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
    ]
}
