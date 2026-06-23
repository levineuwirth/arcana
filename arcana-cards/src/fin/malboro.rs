//! Malboro — `{4}{B}{B}` 4/4 Plant Horror.
//! Bad Breath — When this creature enters, each opponent discards a card,
//! loses 2 life, and exiles the top three cards of their library.
//! Swampcycling {2} (typecycling → generic Cycling with its printed cost).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Malboro");
    let plant = reg.interner_mut().intern("Plant");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: bad_breath,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Each opponent discards a card, loses 2 life, and exiles (mills to
/// exile — modeled here as mill of) the top three cards of their library.
fn bad_breath(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
        effects.push(Effect::LoseLife { player: opp, amount: 2 });
        // GAP: "exiles the top three cards of their library" — no
        // exile-from-top-of-library effect; Mill (to graveyard) is the
        // nearest expressible primitive for top-of-library removal.
        effects.push(Effect::Mill { player: opp, count: 3 });
    }
    vec![Effect::Sequence(effects)]
}
