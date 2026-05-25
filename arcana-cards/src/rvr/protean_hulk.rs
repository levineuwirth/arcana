//! Protean Hulk — `{5}{G}{G}` 6/6 green creature (Beast).
//! "When this creature dies, search your library for any number of creature
//! cards with total mana value 6 or less, put them onto the battlefield,
//! then shuffle."
//!
//! GAP: "any number of creature cards with total mana value 6 or less" —
//! TutorToBattlefield supports a single filter and single card; no
//! multi-card selection with aggregate CMC constraint. Emitting a single
//! TutorToBattlefield with max_cmc 6 as best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Protean Hulk");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "any number of creature cards with total CMC 6 or less" —
    // no multi-card tutor with aggregate CMC constraint; emitting single
    // TutorToBattlefield with with_max_cmc(6) as best effort.
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: arcana_core::targets::ObjectFilter::creature().with_max_cmc(6),
        tapped: false,
    }]
}
