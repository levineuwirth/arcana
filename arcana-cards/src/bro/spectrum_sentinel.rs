//! Spectrum Sentinel — `{1}` 1/2 Artifact Creature — Soldier. Colorless.
//!
//! Oracle text:
//! * Protection from multicolored
//! * Whenever a nonbasic land an opponent controls enters, you gain 1 life.
//!
//! Decomposition:
//! * GAP: "Protection from multicolored" — Protection is NOT in the usable
//!   `KeywordAbility` surface, so `keywords: vec![]`.
//! * ZoneChange trigger → one `TriggeredAbilityDef` watching a nonbasic land
//!   an opponent controls entering the battlefield; gains its controller 1
//!   life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spectrum Sentinel");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let nonbasic_land = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .controlled_by(ControllerConstraint::Opponent);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: nonbasic_land,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: gain_one_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_one_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}
