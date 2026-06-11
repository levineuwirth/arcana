//! In Search of Greatness — `{G}{G}` enchantment (Kaldheim, 2021).
//! "At the beginning of your upkeep, you may cast a permanent spell
//! from your hand with mana value equal to 1 plus the greatest mana
//! value among other permanents you control without paying its mana
//! cost. If you don't, scry 1."
//!
//! The upkeep trigger is wired; the free-cast permission (cast from
//! hand without paying, with a board-derived exact-mana-value gate)
//! has no catalog Effect — that half is an honest GAP and the
//! fall-back scry 1 is emitted (fidelity note: the scry should happen
//! only when the cast is declined).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("In Search of Greatness");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: search_greatness,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…If you don't, scry 1." (The free-cast half is GAP'd; the scry is
/// emitted unconditionally as the documented fallback.)
fn search_greatness(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may cast a permanent spell from your hand with mana value
    // equal to 1 plus the greatest mana value among other permanents you
    // control without paying its mana cost" — free-cast-from-hand
    // permissions and the greatest-mana-value-among-permanents read are
    // not expressible with any catalog Effect; only the scry-1 fallback is
    // modeled (and it fires every upkeep instead of only on decline).
    vec![Effect::Scry {
        player: trig.controller,
        count: 1,
    }]
}
