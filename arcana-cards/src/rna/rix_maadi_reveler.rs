//! Rix Maadi Reveler — `{1}{R}` 2/2 Human Shaman.
//!
//! Oracle:
//! * Spectacle {2}{B}{R}.
//! * When this creature enters, discard a card, then draw a card. If this
//!   creature's spectacle cost was paid, instead discard your hand, then
//!   draw three cards.
//!
//! Spectacle is an alternative-cost keyword not in the supported keyword
//! surface (GAP). The ETB trigger's base mode (discard 1, then draw 1) is
//! expressible; the "if the spectacle cost was paid" alternate (discard hand,
//! draw three) needs a cost-paid accessor that is not exposed, so the
//! conditional upgrade is GAP'd and the base mode always resolves.

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Rix Maadi Reveler");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Spectacle {2}{B}{R} — alternative-cost keyword, not supported.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
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
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Base mode: discard a card, then draw a card.
    // GAP: "If this creature's spectacle cost was paid, instead discard your
    //   hand, then draw three cards" — no cost-paid accessor; base mode only.
    vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}
