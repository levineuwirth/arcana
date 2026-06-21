//! Guile — `{3}{U}{U}{U}` 6/6 Elemental Incarnation.
//! This creature can't be blocked except by three or more creatures. (GAP.)
//! If a spell or ability you control would counter a spell, instead exile that
//! spell and you may play that card without paying its mana cost. (GAP:
//! replacement effect.)
//! When Guile is put into a graveyard from anywhere, shuffle it into its
//! owner's library. (GAP: no shuffle-into-library effect.)

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
    let name = reg.interner_mut().intern("Guile");
    let elemental = reg.interner_mut().intern("Elemental");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: static — "can't be blocked except by three or more creatures" has no
    // count-gated block-restriction Effect.
    // GAP: static — counter-replacement ("would counter a spell, instead exile
    // that spell and you may play it") is a replacement effect not expressible
    // as a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "put into a graveyard from anywhere" — approximated by the
            // self-dies (battlefield→graveyard) trigger; the from-other-zones
            // breadth is a documented fidelity gap.
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: shuffle_into_library,
            trigger_zones: vec![Zone::Battlefield, Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn shuffle_into_library(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "shuffle it into its owner's library" has no matching Effect variant.
    Vec::new()
}
