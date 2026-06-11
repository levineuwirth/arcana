//! Tocasia's Welcome — `{2}{W}` enchantment.
//! "Whenever one or more creatures you control with mana value 3 or less
//! enter, draw a card. This ability triggers only once each turn."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tocasia's Welcome");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_cmc(3),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: draw_one,
                trigger_zones: vec![Zone::Battlefield],
                // "This ability triggers only once each turn." — the
                // OncePerTurn frequency also folds the "one or more"
                // batching into a single draw.
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…draw a card."
fn draw_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
