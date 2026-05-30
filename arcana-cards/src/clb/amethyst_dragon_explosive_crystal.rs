//! Amethyst Dragon // Explosive Crystal — `{4}{R}{R}` red Dragon creature 4/4 with Flying and Haste.
//! Adventure face: "Explosive Crystal" (`{4}{R}` sorcery, "Explosive Crystal deals 4 damage
//! divided as you choose among any number of targets.")
//!
//! # GAPs
//! - "Divided damage among any number of targets": split/divided damage is not expressible
//!   in the engine catalog. The adventure effect fn returns Vec::new() with a GAP comment.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Amethyst Dragon");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Explosive Crystal");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Explosive Crystal deals 4 damage divided as you choose among any number of targets.".into(),
        target_requirements: vec![],
        modal: None,
        effect: explosive_crystal_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure),
    )
}

fn explosive_crystal_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Deals 4 damage divided as you choose among any number of targets" —
    // split/divided damage distribution not expressible in engine effect catalog
    Vec::new()
}
