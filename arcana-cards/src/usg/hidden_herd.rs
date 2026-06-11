//! Hidden Herd — `{G}` enchantment (Urza's Saga, 1998).
//! "When an opponent plays a nonbasic land, if this permanent is an
//! enchantment, it becomes a 3/3 Beast creature."
//!
//! A battlefield-bound `ZoneChange` trigger over nonbasic lands an opponent
//! controls ("plays" approximated by enters-the-battlefield). GAPs: the
//! intervening-if enchantment check and the Beast subtype. The animation is
//! AddType + SetBasePT for as long as the source stays on the battlefield.

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
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hidden Herd");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "PLAYS a nonbasic land"; approximated as a
                // nonbasic land entering the battlefield under an opponent's
                // control (also fires for lands put onto the battlefield by
                // effects).
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .without_supertypes(
                            SupertypeSet::new().with(SupertypeSet::BASIC),
                        )
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Battlefield,
                },
                // GAP: intervening-if "if this permanent is an enchantment"
                // — no source-type condition helper; firing unconditionally.
                intervening_if: None,
                effect: become_beast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 3/3 Beast creature."
fn become_beast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Beast creature subtype cannot be added (no add-subtype
    // effect); type and P/T are applied.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 3,
            toughness: 3,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
