//! Isperia the Inscrutable — `{1}{W}{W}{U}{U}` 3/6 Legendary Sphinx.
//! Flying.
//! Whenever Isperia deals combat damage to a player, choose a card name.
//! That player reveals their hand. If a card with the chosen name is
//! revealed this way, search your library for a creature card with
//! flying, reveal it, put it into your hand, then shuffle.
//!
//! Flying is a base keyword. The combat-damage trigger has no expressible
//! payload: there is no "name a card, reveal their hand, then conditional
//! tutor on a match" primitive (NameCardAndExile strips cards rather than
//! gating a search). The trigger fires but its effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Isperia the Inscrutable");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: source_filter cannot pin to this creature alone; uses
            // your-controlled creatures (established precedent), which
            // over-fires vs. "Isperia deals combat damage". The effect is
            // GAP'd anyway, so the over-fire is moot.
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_name_reveal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_name_reveal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose a card name. That player reveals their hand. If a card
    // with the chosen name is revealed, search your library for a creature
    // card with flying and put it into your hand." No name-a-card +
    // reveal-hand + conditional-tutor primitive in the demonstrated API.
    Vec::new()
}
