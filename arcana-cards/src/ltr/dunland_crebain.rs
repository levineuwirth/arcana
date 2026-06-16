//! Dunland Crebain — `{2}{B}` 1/1 Bird Horror with Flying.
//! "When this creature enters, amass Orcs 2."
//!
//! Decomposed as: a keyword line (Flying — Amass is a mechanic emitted by
//! the ETB effect, not a KeywordAbility variant) plus one ETB triggered
//! ability that runs Amass Orcs 2.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dunland Crebain");
    let bird = reg.interner_mut().intern("Bird");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(horror);
    // Pre-intern the Amass subtypes so the effect fn's lookup resolves.
    let _army = reg.interner_mut().intern("Army");
    let _orc = reg.interner_mut().intern("Orc");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_amass_orcs_2,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_amass_orcs_2(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Orc").unwrap_or_default();
    vec![Effect::Amass {
        controller: trig.controller,
        count: 2,
        army_subtype,
        race_subtype,
    }]
}
