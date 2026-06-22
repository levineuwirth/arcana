//! Lizard, Connors's Curse — `{2}{G}{G}` 5/5 Legendary Lizard Villain.
//! Trample.
//! Lizard Formula — When Lizard, Connors's Curse enters, up to one other
//! target creature loses all abilities and becomes a green Lizard creature
//! with base power and toughness 4/4.
//!
//! The "becomes a *Lizard*" subtype change is a GAP (no AddSubtype effect
//! in the demonstrated primitives); the rest of the transformation
//! (lose all abilities, become green, base P/T 4/4) is wired faithfully.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lizard, Connors's Curse");
    let lizard = reg.interner_mut().intern("Lizard");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(villain);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: lizard_formula,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn lizard_formula(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "becomes a Lizard" subtype change — no AddSubtype effect.
    vec![
        Effect::LoseAllAbilities { target: *id, duration: Duration::Permanent },
        Effect::SetColor {
            target: *id,
            colors: ColorSet::green(),
            duration: Duration::Permanent,
        },
        Effect::SetBasePT {
            target: *id,
            power: 4,
            toughness: 4,
            duration: Duration::Permanent,
        },
    ]
}
