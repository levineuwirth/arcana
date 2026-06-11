//! Pure Reflection — `{2}{W}` enchantment.
//! "Whenever a player casts a creature spell, destroy all Reflections.
//! Then that player creates an X/X white Reflection creature token, where
//! X is the mana value of that spell."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pure Reflection");
    let _reflection = reg.interner_mut().intern("Reflection");
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
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: reflect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…destroy all Reflections. Then that player creates an X/X white
/// Reflection creature token, where X is the mana value of that spell."
fn reflect(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Reflection"),
        trig.controller,
    );
    let mut effects = Vec::new();
    if !ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent {
                target: NULL_OBJECT_ID,
            }),
        });
    }
    // GAP: "creates an X/X white Reflection creature token, where X is the
    // mana value of that spell" — the cast spell's mana value is not readable
    // from the trigger, so the dynamic token half is omitted (the destroy-all
    // half above is faithful).
    effects
}
