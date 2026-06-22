//! Curator Beastie — `{4}{G}{G}` 6/6 Beast with Reach.
//!
//! Reach.
//! Colorless creatures you control enter with two additional +1/+1
//!   counters on them.
//! Whenever this creature enters or attacks, manifest dread.
//!
//! Reach and the "enters or attacks" trigger are wired. The "enters or
//! attacks" line decomposes into two triggered abilities (a self-ETB
//! and a self-attack), each best-effort modeled with `Effect::Manifest`
//! (manifest the top card).
//! GAP: the "manifest DREAD" nuance (look at the top two, put one face
//! down and the other into the graveyard) — only single-card Manifest
//! is expressible.
//! GAP: the static "Colorless creatures you control enter with two
//! additional +1/+1 counters" — an enters-with-counters replacement.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Curator Beastie");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: static "Colorless creatures you control enter with two
    // additional +1/+1 counters on them." — enters-with replacement.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: manifest_dread,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: manifest_dread,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn manifest_dread(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "manifest DREAD" (look at top two, one face down, one to
    // graveyard) — only single-card Manifest is expressible.
    vec![Effect::Manifest {
        player: trig.controller,
    }]
}
