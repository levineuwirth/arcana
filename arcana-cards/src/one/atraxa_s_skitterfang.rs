//! Atraxa's Skitterfang — `{3}` 2/2 colorless Phyrexian Insect Artifact
//! Creature. "This creature enters with three oil counters on it." (GAP'd
//! — no shown enters-with-counters builder.)
//! "At the beginning of combat on your turn, you may remove an oil counter
//! from this creature. When you do, target creature you control gains your
//! choice of flying, vigilance, deathtouch, or lifelink until end of turn."
//! (GAP'd — the optional remove-a-counter cost gating a reflexive keyword
//! grant, plus the four-way keyword choice, is not expressible.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Atraxa's Skitterfang");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(insect);

    // GAP: "enters with three oil counters" — no shown enters-with-counters
    // builder for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: combat_oil_grant,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_oil_grant(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may remove an oil counter ... when you do, target creature
    // you control gains your choice of flying/vigilance/deathtouch/lifelink"
    // — the optional counter-removal cost gating a reflexive keyword grant,
    // and the four-way player keyword choice, are not expressible.
    Vec::new()
}
