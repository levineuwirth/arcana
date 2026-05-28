//! Pearl Lake Warden // Nesting Instinct — `{3}{U}` // `{2}{G}` blue Adventure creature.
//! Creature: 4/5 Dragon. "As long as this card is the top card of your library, you may look at it and cast it." Flying.
//! Adventure (Nesting Instinct — Sorcery): Seek a land card and put it onto the battlefield.
//! GAP: "as long as this card is the top card of your library, you may look at and cast it" — continuous effect from library not in catalog.
//! GAP: "seek a land card" — Seek (random search) not in catalog; using TutorToBattlefield as approximation.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pearl Lake Warden");
    let adv_name = reg.interner_mut().intern("Nesting Instinct");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Seek a land card and put it onto the battlefield.".into(),
        target_requirements: vec![],
        modal: None,
        effect: nesting_instinct_resolve,
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

fn nesting_instinct_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek" (random search) not in catalog; using TutorToBattlefield as approximation
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        tapped: false,
    }]
}
