//! Vexing Scuttler — `{8}` 4/5 Eldrazi Crab.
//!
//! Emerge {6}{U}.
//! When you cast this spell, you may return target instant or sorcery card from
//! your graveyard to your hand.
//!
//! GAP (keyword): Emerge {6}{U} is an alternative casting cost; no
//! `KeywordAbility` variant exists for it (Wretched Gryff precedent).
//! The cast trigger is wired as a self `SpellCast` trigger from the Stack zone.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vexing Scuttler");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(crab);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: cast_return_spell,
            trigger_zones: vec![Zone::Stack],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
                        .controlled_by(ControllerConstraint::You),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

/// When you cast this spell: you may return the targeted instant/sorcery card
/// from your graveyard to your hand.
fn cast_return_spell(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
