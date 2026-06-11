//! Family's Favor — `{2}{G}` enchantment.
//! "Whenever you attack, put a shield counter on target attacking
//! creature. Until end of turn, it gains 'Whenever this creature
//! deals combat damage to a player, remove a shield counter from it.
//! If you do, draw a card.'"
//!
//! GAP: "whenever you attack" (once per attack declaration) is
//! approximated by `CreatureAttacks` over your creatures, which fires
//! once PER attacker. GAP: "target attacking creature" — the
//! demonstrated target surface has no attacking-only constraint, so
//! any creature is targetable. The counter + granted trigger are
//! wired; the granted trigger's "if you do" reflexive is modeled as
//! remove-then-draw.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Family's Favor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "whenever you attack" fires once per combat in
                // oracle terms; CreatureAttacks fires per attacker.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: bestow_favor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target ATTACKING creature" — no attacking-only
                // target constraint in the demonstrated surface.
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

/// "…put a shield counter on target attacking creature. Until end of
/// turn, it gains [the combat-damage draw trigger]."
fn bestow_favor(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Shield,
            count: 1,
        },
        Effect::GrantTriggeredAbility {
            target: *id,
            ability: Box::new(TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: granted_remove_shield_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            duration: Duration::EndOfTurn,
        },
    ]
}

/// Granted: "Whenever this creature deals combat damage to a player,
/// remove a shield counter from it. If you do, draw a card."
/// GAP: the "if you do" reflexive gate is modeled as an unconditional
/// remove-then-draw.
fn granted_remove_shield_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::RemoveCounters {
            target: trig.source,
            kind: CounterKind::Shield,
            count: 1,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
