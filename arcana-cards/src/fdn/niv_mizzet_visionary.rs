//! Niv-Mizzet, Visionary — `{4}{U}{R}` 5/5 Legendary Creature — Dragon
//! Wizard.
//!
//! Oracle:
//! * Flying.
//! * "You have no maximum hand size." — a pure static replacement of
//!   the cleanup discard rule; no demonstrated primitive expresses it.
//!   GAP.
//! * "Whenever a source you control deals noncombat damage to an
//!   opponent, you draw that many cards." → a `DamageDealt` trigger
//!   (sources you control → a player), drawing `damage_amount`. The
//!   damaged player is gated to an opponent in the effect. NOTE: there
//!   is no noncombat-only flag (`combat_only: false` matches any
//!   damage), so combat damage also triggers — a documented fidelity
//!   over-fire.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP static: "You have no maximum hand size."

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Niv-Mizzet, Visionary");
    let dragon = reg.interner_mut().intern("Dragon");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: draw_that_many,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_that_many(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(damaged) = trig.damaged_player() else { return Vec::new(); };
    // Only "to an opponent" qualifies.
    if !script::opponents(state, trig.controller).contains(&damaged) {
        return Vec::new();
    }
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: trig.controller, count: n }]
}
