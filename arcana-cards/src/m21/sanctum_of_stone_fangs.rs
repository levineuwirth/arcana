//! Sanctum of Stone Fangs — `{1}{B}` Legendary Enchantment — Shrine.
//! "At the beginning of your first main phase, each opponent loses X
//! life and you gain X life, where X is the number of Shrines you
//! control."
//!
//! Fidelity note: PhaseBegins{PreCombatMain} fires at the precombat
//! main phase, which is the first main phase of the turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanctum of Stone Fangs");
    let shrine = reg.interner_mut().intern("Shrine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shrine);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PreCombatMain,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: drain_per_shrine,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…each opponent loses X life and you gain X life, where X is the
/// number of Shrines you control."
fn drain_per_shrine(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(shrine) = reg.interner().lookup("Shrine") else {
        return Vec::new();
    };
    let filter = ObjectFilter::permanent()
        .with_subtypes_any(vec![shrine])
        .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &filter, trig.controller);
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife {
            player: p,
            amount: x,
        })
        .collect();
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: x,
    });
    vec![Effect::Sequence(effects)]
}
