//! Boreas Charger — `{2}{W}` 2/1 Pegasus with Flying.
//!
//! Oracle:
//! * Flying (keyword line).
//! * When this creature leaves the battlefield, choose an opponent who
//!   controls more lands than you. Search your library for a number of
//!   Plains cards equal to the difference, reveal those cards, put one
//!   onto the battlefield tapped and the rest into your hand, then
//!   shuffle.
//!   GAP: the dynamic count (difference in land totals between a chosen
//!   opponent and you) combined with a split-destination tutor (one to
//!   battlefield tapped, the rest to hand) is not expressible — the
//!   leaves-battlefield trigger is emitted with an empty body.

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
    let name = reg.interner_mut().intern("Boreas Charger");
    let pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfLeavesBattlefield,
            intervening_if: None,
            effect: search_plains,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn search_plains(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic-count Plains tutor split between battlefield (tapped)
    // and hand, gated on a chosen opponent's land lead, is not expressible.
    Vec::new()
}
