//! Carpet of Flowers — `{G}` enchantment.
//! "At the beginning of each of your main phases, if you haven't added
//! mana with this ability this turn, you may add X mana of any one color,
//! where X is the number of Islands target opponent controls."
//!
//! Modeled as TWO triggers — one per main phase (PreCombatMain and
//! PostCombatMain), since PhaseBegins matches a single phase.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Carpet of Flowers");
    let _island = reg.interner_mut().intern("Island");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    let target_opponent = || TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if — "if you haven't added mana with this
                // ability this turn" has no once-per-turn / event-history
                // predicate in the conditions catalog; fires each main phase.
                intervening_if: None,
                effect: add_island_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_opponent()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PostCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_island_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_opponent()],
            }),
    )
}

/// "…you may add X mana of any one color, where X is the number of Islands
/// target opponent controls."
fn add_island_mana(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let filter = script::subtype_filter(reg, "Island")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, *p) as usize;
    if n == 0 {
        return Vec::new();
    }
    // GAP: fidelity — "any one color" is a player choice with no primitive;
    // green (this card's color) is produced instead.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, trig.source); n],
    }]
}
