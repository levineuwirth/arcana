//! Kinetic Augur — `{3}{R}` */4 Human Shaman with Trample.
//!
//! Trample.
//! Kinetic Augur's power is equal to the number of instant and sorcery cards
//! in your graveyard.
//! When this creature enters, discard up to two cards, then draw that many
//! cards.
//!
//! Trample is wired and the `*` power is set via PtValue::Star. The CDA
//! ("power equal to instant/sorcery cards in your graveyard") is a continuous
//! characteristic-defining static with no trigger/activated form (GAP). The
//! ETB "discard up to two, then draw that many" couples a variable draw count
//! to a player-chosen discard count, which is not expressible (GAP).

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
    let name = reg.interner_mut().intern("Kinetic Augur");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: CDA "power = instant/sorcery cards in your graveyard" — set as
        // a `*` placeholder.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard up to two cards, then draw that many cards" — the draw
    // count is the player-chosen discard count; this coupling is not
    // expressible.
    Vec::new()
}
