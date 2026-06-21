//! Custodi Squire — `{4}{W}` 3/3 Spirit Cleric with Flying.
//!
//! Oracle:
//! * Flying.
//! * Will of the council — When this creature enters, starting with you,
//!   each player votes for an artifact, creature, or enchantment card in your
//!   graveyard. Return each card with the most votes (or tied) to your hand.
//!
//! Flying is a base characteristic. The ETB is a "will of the council" voting
//! effect: there is no voting primitive and no way to express "return cards
//! with the most votes", so the ETB effect is GAP'd while the ETB trigger
//! shape is recorded.

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
    let name = reg.interner_mut().intern("Custodi Squire");
    let spirit = reg.interner_mut().intern("Spirit");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_council_vote,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_council_vote(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Will of the council — each player votes for an artifact/creature/
    // enchantment card in your graveyard; return each card with the most votes
    // to your hand" — no voting primitive; the most-votes return is not
    // expressible.
    Vec::new()
}
