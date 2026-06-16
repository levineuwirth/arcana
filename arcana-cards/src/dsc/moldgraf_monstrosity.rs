//! Moldgraf Monstrosity — `{4}{G}{G}{G}` 8/8 green Insect with Trample.
//! When this creature dies, exile it, then return two creature cards at random
//! from your graveyard to the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moldgraf Monstrosity");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_reanimate_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Exile this card, then return two creature cards at random from your
/// graveyard to the battlefield. Reanimate is a non-targeted "return a creature
/// from a graveyard" — two of them models the "two at random" (selection is
/// engine-side).
fn dies_reanimate_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let self_id = trig.dying_object().unwrap_or(trig.source);
    vec![
        Effect::ExileFromGraveyard { target: self_id },
        Effect::Reanimate {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            from_zone: Zone::Graveyard(trig.controller),
        },
        Effect::Reanimate {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            from_zone: Zone::Graveyard(trig.controller),
        },
    ]
}
