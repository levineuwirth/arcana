//! Essence of Orthodoxy — `{3}{W}{W}` 3/3 Phyrexian with Flying.
//! "Whenever this creature or another Phyrexian you control enters,
//! incubate 2." (Incubate / Transform are mechanics, not unit
//! keywords, so the keyword line is just Flying.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Essence of Orthodoxy");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let phyrexian_filter =
        script::subtype_filter(reg, "Phyrexian").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: phyrexian_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: incubate_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn incubate_two(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Incubate {
        controller: trig.controller,
        n: 2,
    }]
}
