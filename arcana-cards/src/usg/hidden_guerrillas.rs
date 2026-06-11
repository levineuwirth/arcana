//! Hidden Guerrillas — `{G}` enchantment.
//! "When an opponent casts an artifact spell, if this permanent is an
//! enchantment, it becomes a 5/3 Soldier creature with trample."
//!
//! Wired on an opponent-cast artifact `SpellCast` trigger; the
//! animation is AddType(Creature) + SetBasePT(5/3) + GrantKeyword
//! (Trample) anchored `WhileSourceOnBattlefield`. The "if this
//! permanent is an enchantment" intervening-if and the Soldier subtype
//! are documented GAPs.

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
    let name = reg.interner_mut().intern("Hidden Guerrillas");
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
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                // GAP: intervening-if — "if this permanent is an
                // enchantment" (a self-type check) has no conditions::
                // helper; the trigger fires unconditionally. Re-applying
                // the animation after the first fire is idempotent.
                intervening_if: None,
                effect: become_soldier,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…it becomes a 5/3 Soldier creature with trample."
fn become_soldier(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Soldier creature subtype cannot be added by an effect
    // (no AddSubtype variant); type, P/T, and trample are applied.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 5,
            toughness: 3,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
