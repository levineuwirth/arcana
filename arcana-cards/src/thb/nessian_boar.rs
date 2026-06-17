//! Nessian Boar — `{3}{G}{G}` 10/6 Boar.
//! "All creatures able to block this creature do so." (static lure — GAP'd)
//! "Whenever this creature becomes blocked by a creature, that creature's
//! controller draws a card."

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Nessian Boar");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: "All creatures able to block this creature do so." — a static
    // must-block (lure) effect, not a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesBlocked,
            intervening_if: None,
            effect: blocker_controller_draws,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn blocker_controller_draws(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(blocker) = trig.other_combatant() else {
        return Vec::new();
    };
    let Some(p) = state.objects.get(blocker).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DrawCards { player: p, count: 1 }]
}
