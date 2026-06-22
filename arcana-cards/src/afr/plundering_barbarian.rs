//! Plundering Barbarian — `{2}{R}` 2/2 Dwarf Barbarian.
//!
//! When this creature enters, choose one —
//! • Smash the Chest — Destroy target artifact.
//! • Pry It Open — Create a Treasure token.
//!
//! Modal "choose one" is a spell-ability feature; a triggered ability has
//! no modal machinery (cf. Pollenbright Druid). We resolve the always-
//! valid, no-target "Pry It Open" mode (create a Treasure) and GAP the
//! targeted "Smash the Chest" (destroy target artifact) mode.

use arcana_core::effects::{CommodityToken, Effect};
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
    let name = reg.interner_mut().intern("Plundering Barbarian");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
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
                effect: etb_pry_it_open,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_pry_it_open(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" on a triggered ability is unmodeled; the
    // targeted "Destroy target artifact" mode is dropped. We resolve the
    // always-legal no-target "Create a Treasure token" mode.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
