//! Treasure Keeper — `{4}` 3/3 colorless Artifact Creature — Construct.
//! "When this creature dies, reveal cards from the top of your library until you reveal a nonland
//! card with mana value 3 or less. You may cast that card without paying its mana cost. Put all
//! revealed cards not cast this way on the bottom of your library in a random order."
//! GAP: "reveal until nonland card with mv ≤ 3, cast for free" — this is approximated via
//! Discover 3, which has a similar reveal-and-cast-for-free mechanic.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Treasure Keeper");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: discover_on_death,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn discover_on_death(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the exact reveal-until-nonland-mv-3-or-less-cast-free mechanic is approximated
    // by Discover 3 (closest available engine primitive — rest-to-bottom differs slightly).
    vec![Effect::Discover {
        player: trig.controller,
        mana_value: 3,
    }]
}
