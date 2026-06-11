//! Answered Prayers — `{1}{W}{W}` enchantment.
//! "Whenever a creature you control enters, you gain 1 life. If this
//! enchantment isn't a creature, it becomes a 3/3 Angel creature with
//! flying in addition to its other types until end of turn."
//!
//! A battlefield-bound `ZoneChange` trigger: gain 1 life, then animate
//! this enchantment (AddType + SetBasePT + flying, end of turn).
//! Documented GAPs: the "isn't a creature" guard is not expressible (the
//! animation is idempotent if already animated this turn) and the Angel
//! subtype cannot be added by an effect.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Answered Prayers");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
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
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_and_awaken,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you gain 1 life. If this enchantment isn't a creature, it becomes a
/// 3/3 Angel creature with flying in addition to its other types until
/// end of turn."
fn gain_and_awaken(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "If this enchantment isn't a creature" guard is not
    // expressible — the animation is applied unconditionally (idempotent
    // within a turn). GAP: the Angel subtype cannot be added (no
    // add-subtype effect).
    vec![
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
    ]
}
