//! Triarch Praetorian — `{1}{B}` 2/1 Artifact Creature — Necron.
//!
//! Flying.
//! Dynastic Codes — When this creature enters from a graveyard, you draw
//!   two cards and you lose 2 life.
//! Unearth {4}{B} (GAP — Unearth is not in the usable keyword surface.)
//!
//! Flying is expressible. "Dynastic Codes" is an enters-from-graveyard
//! trigger modeled as a `ZoneChange` (from Graveyard to Battlefield)
//! filtered to this card by name; its effect draws two cards and loses 2
//! life. The Unearth graveyard-cast mechanic has no expressible
//! keyword/primitive and is GAP'd.

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
    let name = reg.interner_mut().intern("Triarch Praetorian");
    let necron = reg.interner_mut().intern("Necron");
    let this_name = reg.interner_mut().intern("Triarch Praetorian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);

    let self_filter = ObjectFilter {
        name: Some(this_name),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Unearth {4}{B} — not in the usable keyword surface.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: self_filter,
                from: Some(Zone::Graveyard(0)),
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: dynastic_codes,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dynastic_codes(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: trig.controller, count: 2 },
        Effect::LoseLife { player: trig.controller, amount: 2 },
    ])]
}
