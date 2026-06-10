//! Jade Idol — `{4}` artifact (Saviors of Kamigawa).
//! "Whenever you cast a Spirit or Arcane spell, this artifact becomes
//! a 4/4 Spirit artifact creature until end of turn." Animation is
//! modeled as AddType(CREATURE) + SetBasePT 4/4 until end of turn.
//!
//! GAP: gaining the Spirit SUBTYPE is not expressible (AddType covers
//! card types only); the animated creature carries no Spirit subtype.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Jade Idol");
    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_subtypes_any(vec![spirit, arcane]),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: animate_idol,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn animate_idol(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 4/4 Spirit artifact creature" — the Spirit
    // subtype grant is not expressible; types + P/T only.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
    ]
}
