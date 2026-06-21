//! Impulsive Pilferer — `{R}` 1/1 red Goblin Pirate.
//! When this creature dies, create a Treasure token.
//! Encore {3}{R} (graveyard activated ability — not expressible).
//!
//! Scryfall's "Treasure"/"Encore" keyword tags are not KeywordAbility
//! variants, so `keywords: vec![]`. Encore (exile-from-graveyard, mint a
//! token copy per opponent that attacks that opponent) has no engine
//! primitive — GAP.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Impulsive Pilferer");
    let goblin = reg.interner_mut().intern("Goblin");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(pirate);

    let keywords: Vec<KeywordAbility> = vec![];

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords,
        ..Default::default()
    };

    // GAP: Encore {3}{R} (exile from graveyard, mint a token copy per
    // opponent that must attack that opponent, haste, sac at end) — no
    // primitive expresses this.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_make_treasure,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_make_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
