//! Sanctum of Calm Waters — `{3}{U}` legendary enchantment — Shrine.
//! "At the beginning of your first main phase, you may draw X cards,
//! where X is the number of Shrines you control. If you do, discard a
//! card."
//!
//! Precombat-main trigger; X is the live Shrine count
//! (`script::subtype_filter` + `count_matching`). Fidelity note: the
//! "you may" is resolved as always-yes (no free may-gate exists); the
//! discard only attaches when X > 0 ("If you do").

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanctum of Calm Waters");
    let shrine = reg.interner_mut().intern("Shrine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shrine);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: draw_per_shrine,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may draw X cards, where X is the number of Shrines you
/// control. If you do, discard a card."
fn draw_per_shrine(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Shrine")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    // Fidelity: "you may" is resolved as always-yes.
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: n,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
