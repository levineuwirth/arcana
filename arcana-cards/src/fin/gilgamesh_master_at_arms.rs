//! Gilgamesh, Master-at-Arms — `{4}{R}{R}` 6/6 red Legendary Creature —
//! Human Samurai.
//! "Whenever Gilgamesh enters or attacks, look at the top six cards of your
//! library. You may put any number of Equipment cards from among them onto the
//! battlefield. Put the rest on the bottom of your library in a random order.
//! When you put one or more Equipment onto the battlefield this way, you may
//! attach one of them to a Samurai you control."
//! GAP: effect — "look at top N, selectively put Equipment onto battlefield,
//! then attach to a Samurai" is not in the catalog. TutorToBattlefield as
//! approximation; attach portion is a GAP.

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
    let name = reg.interner_mut().intern("Gilgamesh, Master-at-Arms");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_or_attack_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: etb_or_attack_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_or_attack_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 6, put Equipment onto battlefield, attach to Samurai"
    // using TutorToBattlefield as approximation.
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        tapped: false,
    }]
}
