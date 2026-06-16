//! Gitrog, Horror of Zhava — `{2}{B}{G}` 6/6 Legendary Frog Horror with
//! Menace.
//! "At the beginning of each combat, if Gitrog is untapped, any opponent
//! may sacrifice a nontoken creature. If they do, tap Gitrog, then seek a
//! land card and put it onto the battlefield tapped."
//! "Whenever a land enters under your control, it perpetually gains
//! '{B}{G}, {T}, Sacrifice this land: Draw a card.'"
//!
//! Menace is a base keyword. Seek is not an available KeywordAbility and
//! is omitted. Both triggers are wired but their bodies are GAP'd: the
//! first needs an opponent-choice + seek (no seek Effect), the second
//! needs a perpetual ability grant onto entering lands (not modeled).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gitrog, Horror of Zhava");
    let frog = reg.interner_mut().intern("Frog");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: combat_seek,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: land_gains_ability,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_seek(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if Gitrog is untapped" intervening-if (no source-untapped
    // condition helper) + "any opponent may sacrifice a nontoken
    // creature" opponent choice + "seek a land card" (no seek Effect).
    Vec::new()
}

fn land_gains_ability(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "it perpetually gains '{B}{G}, {T}, Sacrifice this land: Draw
    // a card.'" — perpetual activated-ability grant onto another
    // permanent is not modeled.
    Vec::new()
}
