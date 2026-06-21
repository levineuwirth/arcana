//! Woodvine Elemental — `{4}{G}{W}` 4/4 Creature — Elemental.
//!
//! Trample
//! * Parley — Whenever this creature attacks, each player reveals the top
//!   card of their library. For each nonland card revealed this way,
//!   attacking creatures you control get +1/+1 until end of turn. Then each
//!   player draws a card.
//!
//! GAP: the reveal-and-count-nonlands step (and the resulting +1/+1 to each
//! attacking creature you control, scaled by that count) has no expressible
//! primitive — there is no "reveal top card of each library and count
//! nonlands" helper. Only the trailing "each player draws a card" is wired.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Woodvine Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: parley_each_player_draws,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn parley_each_player_draws(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal-top-of-each-library + count nonlands + scaled pump of
    // attacking creatures you control is not expressible. Wiring only the
    // trailing "then each player draws a card".
    let _ = trig;
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect()
}
