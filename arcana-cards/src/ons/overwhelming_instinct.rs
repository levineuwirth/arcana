//! Overwhelming Instinct — `{2}{G}` green Enchantment.
//! "Whenever you attack with three or more creatures, draw a card."
//!
//! Approximation: `CreatureAttacks { creature().controlled_by(You) }` fires
//! once per attacker. `OncePerTurn` limits resolution to the first firing.
//! The intervening-if checks the total attacker count at resolution time
//! (after all attackers are declared); if ≥ 3 the trigger resolves and draws.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Overwhelming Instinct");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: Some(if_three_or_more_attackers),
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

/// "if you are attacking with three or more creatures" — checked at resolution
/// (after all attackers are declared), counting current attacking creatures.
fn if_three_or_more_attackers(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::count_matching(
        s,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .attacking_only(),
        you,
    ) >= 3
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
