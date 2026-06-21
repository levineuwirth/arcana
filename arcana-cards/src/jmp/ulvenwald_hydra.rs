//! Ulvenwald Hydra — `{4}{G}{G}` */* Hydra with Reach.
//! Reach.
//! "Ulvenwald Hydra's power and toughness are each equal to the number
//! of lands you control." (a CDA — GAP'd; PtValue::Star marks the slot.)
//! When this creature enters, you may search your library for a land
//! card, put it onto the battlefield tapped, then shuffle.
//!
//! Reach is a base keyword. The */* defining static cannot be installed
//! via the demonstrated API (it is a CDA) so PtValue::Star marks the
//! slot and the count static is GAP'd. The ETB tutor is fully modeled
//! ("you may" — TutorToBattlefield posts a may-search). The shuffle is
//! automatic on a library search.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulvenwald Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP (CDA): "power and toughness are each equal to the number of lands
    // you control" — the */* defining static cannot be installed via the
    // demonstrated triggered/activated API; PtValue::Star marks the slot.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_tutor_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        tapped: true,
    }]
}
