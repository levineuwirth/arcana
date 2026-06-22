//! Slithermuse — `{2}{U}{U}` 3/3 Elemental.
//!
//! Oracle:
//! * When this creature leaves the battlefield, choose an opponent. If
//!   that player has more cards in hand than you, draw cards equal to
//!   the difference.
//! * Evoke {3}{U}  (GAP — Evoke is not a usable KeywordAbility variant.)
//!
//! The leaves-battlefield trigger is wired. PARTIAL: rather than posting
//! an opponent choice (which can't compute the resulting hand-size
//! difference at resolution), the resolver selects the opponent with the
//! most cards in hand and draws the (clamped) difference vs your hand —
//! the maximizing, faithful interpretation of the payoff.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Slithermuse");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Evoke {3}{U} — not a usable KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: draw_difference,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_difference(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let my_hand = script::hand_size(state, trig.controller);
    let best_opp = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| script::hand_size(state, p))
        .max()
        .unwrap_or(0);
    let diff = best_opp.saturating_sub(my_hand);
    if diff == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: trig.controller, count: diff }]
}
