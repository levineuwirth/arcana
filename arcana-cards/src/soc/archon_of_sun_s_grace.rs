//! Archon of Sun's Grace — `{2}{W}{W}` 3/4 Creature — Archon.
//! Flying, lifelink.
//! Pegasus creatures you control have lifelink.
//! Constellation — Whenever an enchantment you control enters, create a 2/2
//! white Pegasus creature token with flying.
//!
//! GAP: "Pegasus creatures you control have lifelink" — a static continuous
//! ability (Constellation is the keyword label only; not a KeywordAbility
//! variant).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archon of Sun's Grace");
    let archon = reg.interner_mut().intern("Archon");
    let _pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let enchantment_filter = ObjectFilter::new()
        .with_types(TypeLine::ENCHANTMENT.into())
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: enchantment_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: make_pegasus,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_pegasus(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let pegasus = reg.interner().lookup("Pegasus").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: pegasus,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
