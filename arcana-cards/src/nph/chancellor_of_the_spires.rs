//! Chancellor of the Spires — `{4}{U}{U}{U}` 5/7 Creature — Phyrexian Sphinx.
//!
//! Oracle:
//! * You may reveal this card from your opening hand. If you do, at the
//!   beginning of the first upkeep, each opponent mills seven cards. (Opening-
//!   hand reveal ability — not modeled; GAP'd.)
//! * Flying.
//! * When this creature enters, you may cast target instant or sorcery card
//!   from an opponent's graveyard without paying its mana cost.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Chancellor of the Spires");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "You may reveal this card from your opening hand. If you do, at the
    // beginning of the first upkeep, each opponent mills seven cards." —
    // opening-hand (Chancellor) reveal ability is not modeled.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: cast_from_opponent_graveyard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cast_from_opponent_graveyard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may cast target instant or sorcery card from an opponent's
    // graveyard without paying its mana cost" — there is no free-cast-from-
    // graveyard Effect primitive.
    Vec::new()
}
