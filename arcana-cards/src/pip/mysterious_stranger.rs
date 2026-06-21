//! Mysterious Stranger — `{2}{R}{R}` 3/2 Human Rogue with Flash.
//!
//! Oracle:
//! * Flash.
//! * When this creature enters, for each graveyard with an instant or sorcery
//!   card in it, exile target instant or sorcery card from that graveyard. If
//!   two or more cards are exiled this way, choose one of them at random and
//!   copy it. You may cast the copy without paying its mana cost.
//!
//! Flash is a base characteristic. The ETB ability is GAP'd: it declares one
//! variable target per graveyard, then random-selects among the exiled cards,
//! copies the chosen one, and free-casts the copy — a composite (per-graveyard
//! targeting + random pick + copy + free cast) with no expressible Effect.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mysterious Stranger");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_random_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile_random_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: per-graveyard variable targeting of instant/sorcery cards, then a
    // random selection among exiled cards, copy it, and free-cast the copy —
    // not expressible with the available primitives.
    Vec::new()
}
