//! Esper Sojourners — `{W}{U}{B}` 2/3 Artifact Creature — Vedalken Wizard.
//!
//! * "When you cycle this card and when this creature dies, you may tap or
//!   untap target permanent." — the dies half is wired as a `SelfDies`
//!   trigger that targets a permanent. The cycling half has no
//!   `TriggerCondition` variant (no "when you cycle ~" condition), so it is
//!   gapped. The "may tap OR untap" player choice has no single Effect to
//!   express the choice, so the targeted-permanent effect itself is gapped.
//! * Cycling {2}{U} (keyword — engine synthesizes the cycling activation).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esper Sojourners");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}{U}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: the "when you cycle this card" half has no TriggerCondition variant
    // for a cycling event; only the dies half is wired below.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: tap_or_untap_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn tap_or_untap_target(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may tap OR untap" is a player-mode choice with no single
    // Effect (Effect::Tap / Effect::Untap exist but the choice between them,
    // plus the optional "may", is not expressible here).
    Vec::new()
}
