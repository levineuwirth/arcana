//! Darkstar Augur — `{2}{B}` 2/3 Bat Warlock. Flying.
//! Offspring {B} — an additional-cost casting keyword not in the usable surface (GAP'd).
//! "At the beginning of your upkeep, reveal the top card of your library and put
//!  that card into your hand. You lose life equal to its mana value." — the
//!  life-loss is dynamic on the revealed card's mana value, with no script accessor
//!  for the top-of-library card's mana value; the coupled draw+life-loss is GAP'd
//!  rather than emit a misleading draw-only partial.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Darkstar Augur");
    let bat = reg.interner_mut().intern("Bat");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(warlock);

    // GAP: Offspring {B} — additional-cost casting keyword, not in usable surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_reveal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_reveal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: reveal top card → hand, then lose life equal to its mana value. The
    // life-loss amount depends on the revealed card's mana value, which has no
    // script accessor; emitting a bare draw would misrepresent the card.
    Vec::new()
}
