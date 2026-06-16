//! Goblin Recruiter — `{1}{R}` 1/1 Goblin.
//! "When this creature enters, search your library for any number of Goblin cards,
//! reveal them, then shuffle and put those cards on top in any order."
//! GAP: "Search for any number" and "put on top in any order" is not expressible;
//! the engine's TutorToHand fetches exactly one card. Best-effort: search one Goblin.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Recruiter");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: recruit_goblins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn recruit_goblins(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Oracle says "any number of Goblin cards" then "put on top in any order".
    // Engine TutorToHand only fetches one matching card.
    // Best-effort approximation: fetch one Goblin card to hand.
    let filter = script::subtype_filter(reg, "Goblin");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}
