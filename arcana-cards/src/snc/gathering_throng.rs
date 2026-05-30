//! Gathering Throng — `{2}{W}` 3/1 white Human Citizen.
//! "When this creature enters, you may search your library for any number of cards
//! named Gathering Throng, reveal them, put them into your hand, then shuffle."
//! Uses TutorToHand with name-based filter. Note: TutorToHand finds ONE card; searching
//! for "any number" is not expressible (single-take only). Using a single TutorToHand
//! as best-effort approximation.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Gathering Throng");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_copies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_copies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "any number of cards named ~" — TutorToHand is single-take only; searching
    // for multiple copies is not expressible. Using a single tutor as best-effort.
    let nm = reg.interner().lookup("Gathering Throng");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        reveal: true,
    }]
}
