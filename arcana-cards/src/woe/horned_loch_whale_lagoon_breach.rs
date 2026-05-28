//! Horned Loch-Whale // Lagoon Breach — `{4}{U}{U}` // `{1}{U}` blue Adventure creature.
//! Creature: 6/6 Whale. Flash, Ward {2}.
//! This creature enters tapped unless it's your turn.
//! Adventure (Lagoon Breach — Instant): The owner of target attacking creature you don't control puts it on their choice of the top or bottom of their library.
//! GAP: "enters tapped unless it's your turn" — conditional ETB tapped not in EntersWithSpec.
//! GAP: Adventure effect "owner puts on top or bottom of library — owner choice" — PutOnTopOfLibrary exists but player-choice not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horned Loch-Whale");
    let adv_name = reg.interner_mut().intern("Lagoon Breach");
    let whale_sub = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(whale_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "The owner of target attacking creature you don't control puts it on top or bottom of their library.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: lagoon_breach_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure),
    )
}

fn lagoon_breach_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: owner chooses top or bottom — using PutOnTopOfLibrary as approximation
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
