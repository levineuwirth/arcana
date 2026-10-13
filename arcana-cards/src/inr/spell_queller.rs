//! Spell Queller — `{1}{W}{U}` 2/3 Spirit with Flash and Flying.
//! "When this creature enters, exile target spell with mana value 4 or
//! less."
//! "When this creature leaves the battlefield, the exiled card's owner
//! may cast that card without paying its mana cost."
//!
//! The two abilities are linked (CR 607): the exile stamps the card with
//! this creature's battlefield id, and the leave ability finds the card by
//! it. If Spell Queller leaves before its first ability resolves, the leave
//! ability resolves first and finds nothing, and the spell exiled afterwards
//! stays exiled. A spell's chosen X counts toward its mana value (CR 202.3e),
//! and exiling a spell isn't countering it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spell Queller");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_target_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::new().with_max_cmc(4)),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: offer_exiled_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Exile the target spell, linked to this creature.
fn exile_target_spell(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(spell)) = trig.targets.targets.first() else { return Vec::new() };
    vec![Effect::ExileSpell { target: *spell, link_to: Some(trig.source) }]
}

/// Offer the linked card's owner the cast, if it can be cast at all: a
/// spell with no legal target is left in exile without asking.
fn offer_exiled_card(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut linked: Vec<_> = state.objects.iter()
        .filter(|o| o.zone.is_exile() && o.linked_exile_source == Some(trig.source))
        .map(|o| (o.id, o.owner))
        .collect();
    linked.sort_unstable();
    linked.into_iter()
        .filter(|&(id, owner)|
            !arcana_core::legal_actions::casts_without_paying(state, reg, owner, id).is_empty())
        .map(|(id, _)| Effect::OfferCastWithoutPaying { object: id })
        .collect()
}
