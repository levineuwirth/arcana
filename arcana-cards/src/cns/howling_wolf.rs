//! Howling Wolf — `{2}{G}{G}` 2/2 green Creature — Wolf.
//! "When this creature enters, you may search your library for up to three
//! cards named Howling Wolf, reveal them, put them into your hand, then shuffle."
//! GAP: effect — TutorToHand searches for one card; "up to three named" is not
//! directly modeled. Emitting three TutorToHand calls (each optional) as
//! best approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Howling Wolf");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_search_wolves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_search_wolves(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "up to three named Howling Wolf" — TutorToHand approximation;
    // name-specific filter and "up to three" not modeled.
    vec![
        Effect::TutorToHand {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            reveal: true,
        },
    ]
}
