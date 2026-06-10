//! Opal Archangel — `{4}{W}` enchantment (Urza's Saga, 1998). "When an
//! opponent casts a creature spell, if this permanent is an enchantment,
//! it becomes a 5/5 Angel creature with flying and vigilance."
//!
//! The Opal animation cycle. The trigger (an opponent casts a creature
//! spell) is wired faithfully; the animation is modeled as
//! `AddType` (creature) + `SetBasePT` (5/5) + flying + vigilance grants
//! with `Duration::WhileSourceOnBattlefield`. Documented GAPs: the
//! "if this permanent is an enchantment" intervening-if has no
//! source-type condition helper (so the trigger can re-fire and
//! re-apply the same animation — harmless but unfaithful), and the
//! Angel subtype cannot be added (no subtype-adding effect).

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
    let name = reg.interner_mut().intern("Opal Archangel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if "if this permanent is an
                // enchantment" — no conditions:: helper tests the
                // source's own types; left None (the animation re-fire
                // is idempotent).
                intervening_if: None,
                effect: awaken_archangel,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "it becomes a 5/5 Angel creature with flying and vigilance."
fn awaken_archangel(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Angel subtype cannot be added (no subtype-adding effect);
    // "becomes" should be a permanent one-shot — approximated with
    // Duration::WhileSourceOnBattlefield.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 5,
            toughness: 5,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
