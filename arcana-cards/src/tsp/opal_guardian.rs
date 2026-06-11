//! Opal Guardian — `{W}{W}{W}` enchantment.
//! "When an opponent casts a creature spell, if this permanent is an
//! enchantment, this enchantment becomes a 3/4 Gargoyle creature with
//! flying and protection from red."
//!
//! Wired on an opponent-cast creature `SpellCast` trigger; the
//! animation is AddType(Creature) + SetBasePT(3/4) + GrantKeyword
//! (Flying) anchored `WhileSourceOnBattlefield`. The "if this permanent
//! is an enchantment" intervening-if, the Gargoyle subtype, and
//! protection from red are documented GAPs.

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
    let name = reg.interner_mut().intern("Opal Guardian");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{W}").expect("valid cost")),
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
                // GAP: intervening-if — "if this permanent is an
                // enchantment" (a self-type check) has no conditions::
                // helper; the trigger fires unconditionally. Re-applying
                // the animation is idempotent.
                intervening_if: None,
                effect: become_gargoyle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…becomes a 3/4 Gargoyle creature with flying and protection from red."
fn become_gargoyle(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Gargoyle subtype cannot be added by an effect, and
    // "protection from red" is not an available KeywordAbility; type,
    // P/T, and flying are applied.
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 3,
            toughness: 4,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}
