//! Combustible Gearhulk — `{4}{R}{R}` 6/6 Artifact Creature — Construct.
//!
//! Rules text:
//! * First strike
//! * When this creature enters, target opponent may have you draw three cards.
//!   If the player doesn't, you mill three cards, then this creature deals damage
//!   to that player equal to the total mana value of those cards.
//!
//! First strike is faithful (the Scryfall "Mill" keyword is not a KeywordAbility
//! variant and is not a separate ability). The ETB trigger is GAP'd: it is an
//! opponent-made branch (no cost, so not OptionalPayment) whose punishment side
//! mills three cards and deals damage equal to "the total mana value of those
//! cards", which can't be computed after the mill.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Combustible Gearhulk");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_draw_or_mill_burn,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_opponent()],
        }),
    )
}

fn etb_draw_or_mill_burn(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: opponent-chosen branch ("may have you draw three cards; otherwise you
    //       mill three, then deal damage equal to the total mana value of those
    //       cards"). It's a no-cost decision (not OptionalPayment), and the
    //       milled cards' total mana value can't be inspected after the mill.
    Vec::new()
}
