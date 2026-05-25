//! Winota, Joiner of Forces — `{2}{R}{W}` 4/4 red-white Legendary Human Warrior.
//! "Whenever a non-Human creature you control attacks, look at the top six cards of your library.
//! You may put a Human creature card from among them onto the battlefield tapped and attacking.
//! It gains indestructible until end of turn. Put the rest on the bottom in a random order."
//! GAP: "put onto battlefield tapped and attacking" + random bottom order not in catalog;
//! using TutorToBattlefield as closest.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::effects::KeywordAbility;
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
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Winota, Joiner of Forces");
    let _human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(_human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: attack_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: filter "non-Human" not available in trigger filter
    // GAP: "look at top 6, may put Human onto battlefield tapped+attacking, rest random bottom"
    let human_filter = script::subtype_filter(reg, "Human");
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: human_filter,
        tapped: true,
    }]
}
