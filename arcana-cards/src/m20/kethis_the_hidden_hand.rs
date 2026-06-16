//! Kethis, the Hidden Hand — `{W}{B}{G}` 3/4 legendary Elf Advisor.
//! "Legendary spells you cast cost {1} less to cast." "Exile two
//! legendary cards from your graveyard: Until end of turn, each
//! legendary card in your graveyard gains 'You may play this card from
//! your graveyard.'"
//!
//! Neither ability is expressible: the cost-reduction static has no
//! triggered/activated form, and the activated ability's cost ("exile
//! two legendary cards from your graveyard") has no ActivationCost field
//! and its effect (grant "play from graveyard" to graveyard cards) has
//! no Effect variant. Only the bones are wired.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kethis, the Hidden Hand");
    let elf = reg.interner_mut().intern("Elf");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Legendary spells you cast cost {1} less" — no
    // cost-reduction static expressible as a triggered/activated ability.
    // GAP: "Exile two legendary cards from your graveyard: …" — no
    // exile-from-graveyard ActivationCost field, and no Effect to grant
    // "play from graveyard" to graveyard cards.

    reg.register(CardDefinition::new(name, chars))
}
