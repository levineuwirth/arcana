//! Faceless Agent — `{3}` 2/2 Shapeshifter.
//! Changeling.
//! When Faceless Agent enters the battlefield, seek a creature card of the
//! most prevalent creature type in your library.
//!
//! Changeling is a base keyword. The ETB "seek" is GAP'd: there is no
//! Effect::Seek primitive, and "most prevalent creature type in your
//! library" has no script helper.

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
    let name = reg.interner_mut().intern("Faceless Agent");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_seek,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_seek(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek a creature card of the most prevalent creature type in your
    // library" — no Effect::Seek primitive and no "most prevalent type"
    // script helper.
    Vec::new()
}
