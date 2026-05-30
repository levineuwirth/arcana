//! Exdeath, Void Warlock // Neo Exdeath, Dimension's End
//!
//! Front face: Legendary Creature — Spirit Warlock, {1}{B}{G}, 3/3.
//! When Exdeath enters, you gain 3 life.
//! At the beginning of your end step, if there are six or more permanent cards in
//! your graveyard, transform Exdeath.
//!
//! Back face: Legendary Creature — Spirit Avatar, Trample.
//! Neo Exdeath's power is equal to the number of permanent cards in your graveyard.
//! GAP: Back-face dynamic power (equal to number of permanent cards in graveyard) is a
//! continuous effect layer modification, not modeled — back face registers 0/0 P/T as a
//! placeholder.
//! GAP: Back-face-only triggered ability not modeled.
//! GAP: "If there are six or more permanent cards in your graveyard" intervening-if uses
//! graveyard permanent count; no intervening_if closure provided — trigger fires unconditionally
//! (engine debt: intervening-if needs state access; closest available approximation).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    ControllerConstraint, PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exdeath, Void Warlock");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);
    subtypes.0.insert(warlock_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Neo Exdeath, Dimension's End");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let spirit_sub2 = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub2);
    back_subtypes.0.insert(avatar_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Trample],
            // GAP: power is dynamic (equal to number of permanent cards in graveyard).
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: you gain 3 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Beginning of end step: if 6+ permanent cards in graveyard, transform.
            // GAP: intervening-if "six or more permanent cards in graveyard" not expressible;
            // trigger fires unconditionally each end step (owner must decide at resolution).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_gain_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 3,
    }]
}

fn end_step_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
