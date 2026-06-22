//! Klothys, God of Destiny — `{1}{R}{G}` 4/5 Legendary Enchantment Creature —
//! God, with Indestructible.
//!
//! Indestructible
//! As long as your devotion to red and green is less than seven, Klothys isn't a
//! creature.
//! At the beginning of your first main phase, exile target card from a
//! graveyard. If it was a land card, add {R} or {G}. Otherwise, you gain 2 life
//! and Klothys deals 2 damage to each opponent.
//!
//! Indestructible is a base keyword.
//! GAP: the devotion "isn't a creature" conditional type-removing static has no
//! primitive in this surface.
//! The first-main-phase trigger exiles a targeted card from a graveyard (wired).
//! GAP: the "if it was a land card → add R or G, otherwise gain 2 life + deal 2
//! to each opponent" branch depends on the exiled card's type, which is not
//! observable after exile — the conditional payoff is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Klothys, God of Destiny");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: exile_card_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn exile_card_from_graveyard(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: branch on whether the exiled card was a land — not observable; the
    // "add R or G / gain 2 life + deal 2 to each opponent" payoff is omitted.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
