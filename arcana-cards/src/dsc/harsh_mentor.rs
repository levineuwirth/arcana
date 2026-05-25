//! Harsh Mentor — `{1}{R}` 2/2 Human Cleric. "Whenever an opponent
//! activates an ability of an artifact, creature, or land on the
//! battlefield, if it isn't a mana ability, this creature deals 2
//! damage to that player."
//!
//! GAP: trigger — the engine has no `TriggerCondition` variant for
//! "an opponent activates an ability" (CR 602 activated-ability
//! events). The closest available variant is `SpellCast`, which is
//! semantically wrong (it fires on spells, not activations), but the
//! catalog forbids inventing a `GameEvent` arm. The intervening-if
//! ("if it isn't a mana ability") is also not modeled and is left as
//! `None`. The effect body is authored faithfully so that if/when an
//! `AbilityActivated` trigger lands, only the condition needs
//! swapping.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harsh Mentor");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no `AbilityActivated` variant exists;
                // `SpellCast` is the closest opponent-action event but is
                // semantically incorrect. The "artifact/creature/land"
                // source filter and the "isn't a mana ability"
                // intervening-if are also unmodeled.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: deal_two_to_triggering_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "This creature deals 2 damage to that player." `that player` is the
/// activator of the ability (or, under the GAP, the caster of the
/// spell that fired the surrogate trigger).
fn deal_two_to_triggering_player(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.triggering_caster() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(p),
        amount: 2,
        source: trig.source,
    }]
}
