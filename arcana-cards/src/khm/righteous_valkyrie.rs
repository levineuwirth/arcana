//! Righteous Valkyrie — `{2}{W}` 2/4 Creature — Angel Cleric.
//! Flying.
//! "Whenever another Angel or Cleric you control enters, you gain life
//! equal to that creature's toughness."
//! Static: "As long as you have at least 7 life more than your starting
//! life total, creatures you control get +2/+2." — GAP (continuous
//! life-gated anthem static; no triggered/activated form).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Righteous Valkyrie");
    let angel = reg.interner_mut().intern("Angel");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(cleric);

    let enters_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![angel, cleric]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP (static): life-gated +2/+2 anthem.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: enters_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: gain_life_equal_toughness,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_life_equal_toughness(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    let n = script::toughness_of(state, id).max(0) as u32;
    vec![Effect::GainLife {
        player: trig.controller,
        amount: n,
    }]
}
