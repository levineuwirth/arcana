//! Progenitus — `{W}{W}{U}{U}{B}{B}{R}{R}{G}{G}` 10/10 Legendary Hydra Avatar.
//! "Protection from everything. (keyword not in catalog — GAP'd)
//!  If Progenitus would be put into a graveyard from anywhere, reveal
//!  Progenitus and shuffle it into its owner's library instead. (replacement
//!  effect — GAP'd)"
//!
//! Only the bones are expressible; both lines are a non-expressible keyword and
//! a replacement effect respectively, so this is a bones-only card.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Progenitus");
    let hydra = reg.interner_mut().intern("Hydra");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{U}{U}{B}{B}{R}{R}{G}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        // GAP: "Protection from everything" — Protection is not in the usable
        // KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "If Progenitus would be put into a graveyard from anywhere, reveal
    // it and shuffle it into its owner's library instead" is a replacement
    // effect — not expressible as a triggered/activated ability.

    reg.register(CardDefinition::new(name, chars))
}
