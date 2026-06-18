//! MagicConsecrated Sphinx — `{4}{U}{U}` 4/6 Sphinx with Flying.
//! Whenever an opponent draws a card, choose one that hasn't been chosen:
//! make a Gamer token / Investigate / make a Treasure / exile-and-return.
//!
//! The "choose one that hasn't been chosen" modal selection is not
//! expressible for a triggered ability (modal dispatch exists only for spell
//! abilities). We fire the trigger and emit the Investigate (Clue) mode as a
//! faithful single payoff; the once-per-mode choice is GAP'd.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("MagicConsecrated Sphinx");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: choose_one_mode,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn choose_one_mode(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "choose one that hasn't been chosen" modal selection (with the
    // Gamer-token / Treasure / exile-return alternatives) is not expressible
    // for triggered abilities. Emit the Investigate (Clue) mode faithfully.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
