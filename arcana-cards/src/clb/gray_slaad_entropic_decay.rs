//! Gray Slaad // Entropic Decay — `{2}{B}` // `{1}{B}` black Adventure creature.
//! Creature: 4/1 Frog Horror. "As long as there are four or more creature cards in your graveyard, menace and deathtouch."
//! Adventure (Entropic Decay — Sorcery): Mill four cards.
//! GAP: "as long as there are 4+ creature cards in your graveyard, gains menace and deathtouch" — conditional static keyword not in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gray Slaad");
    let adv_name = reg.interner_mut().intern("Entropic Decay");
    let frog_sub = reg.interner_mut().intern("Frog");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog_sub);
    subtypes.0.insert(horror_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Mill four cards.".into(),
        target_requirements: vec![],
        modal: None,
        effect: entropic_decay_resolve,
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

fn entropic_decay_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill { player: entry.controller, count: 4 }]
}
