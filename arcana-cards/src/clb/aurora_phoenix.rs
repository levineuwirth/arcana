//! Aurora Phoenix — `{4}{R}{R}` 5/3 Phoenix.
//!
//! Flying
//! Cascade (GAP: keyword not on the usable surface for this card class)
//! Whenever you cast a spell with cascade, return this card from your
//! graveyard to your hand.
//!
//! The recursion trigger is wired as a "whenever you cast a spell" trigger
//! while in the graveyard; the "with cascade" restriction on the cast spell
//! is not expressible via `ObjectFilter`, so it over-fires (documented).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aurora Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Cascade — keyword not on the usable KeywordAbility surface.
        keywords: vec![KeywordAbility::Flying],
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "with cascade" restriction on the cast spell is not
                // expressible via ObjectFilter — fires on any spell you cast.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: return_self_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn return_self_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: trig.source }]
}
