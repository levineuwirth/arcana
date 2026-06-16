//! Dread Defiler — `{6}{B}` 6/8 Eldrazi with Devoid (colorless).
//! Its only ability ("{3}{C}, Exile a creature card from your graveyard: Target
//! opponent loses life equal to the exiled card's power") has an
//! exile-a-graveyard-card activation cost that ActivationCost cannot express
//! (no exile-from-graveyard cost field; OptionalPaymentKind has only Mana/Life),
//! and its payload scales on that exiled card's power, which is unknowable
//! without the cost mechanism — so the whole ability is GAP'd. Bones-only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dread Defiler");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    // Devoid: the card is colorless.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    // GAP: "{3}{C}, Exile a creature card from your graveyard: Target opponent
    //      loses life equal to the exiled card's power." — no exile-from-graveyard
    //      activation-cost field, and the life loss scales on the exiled card's
    //      power (unknowable without the cost mechanism).
    reg.register(CardDefinition::new(name, chars))
}
