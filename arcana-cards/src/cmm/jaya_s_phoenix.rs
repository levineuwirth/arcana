//! Jaya's Phoenix — `{4}{R}` 3/3 Creature — Phoenix.
//! Flying, haste.
//! "Whenever this creature deals combat damage to a player or
//!   planeswalker, copy the next loyalty ability you activate this turn
//!   when you activate it. You may choose new targets for the copy."
//! "Whenever you cast a planeswalker spell, you may return this card from
//!   your graveyard to the battlefield."
//!
//! The combat-damage trigger's payload (deferred copy of the next
//! activated loyalty ability) has no demonstrated primitive and is GAP'd.
//! The cast-planeswalker recursion trigger fires from the graveyard and
//! returns this card to the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaya's Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let pw_filter = ObjectFilter::new()
        .with_types(TypeLine::PLANESWALKER.into());

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: arcana_core::targets::TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: copy_next_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(pw_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: return_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_next_loyalty(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy the next loyalty ability you activate this turn when you
    // activate it." No deferred copy-next-activated-ability primitive.
    Vec::new()
}

fn return_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may return this card from your graveyard to the battlefield."
    // The optional choice is a resolution-time decision; the return is
    // emitted faithfully.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
