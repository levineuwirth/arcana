//! Innocuous Insect — `{1}{U}{U}` 2/1 Eldrazi Insect.
//! Buyback {1}{U} (GAP'd — Buyback is not a usable KeywordAbility).
//! Flash, Flying.
//! When you cast this spell, draw a card.

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
    let name = reg.interner_mut().intern("Innocuous Insect");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(insect);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Buyback {1}{U} has no usable KeywordAbility variant.
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new()),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
