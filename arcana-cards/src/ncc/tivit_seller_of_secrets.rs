//! Tivit, Seller of Secrets — `{3}{W}{U}{B}` 6/6 Legendary Sphinx Rogue.
//!
//! * Flying, ward {3} — keywords.
//! * "Council's dilemma — Whenever Tivit enters or deals combat damage to a
//!   player, starting with you, each player votes for evidence or bribery.
//!   For each evidence vote, investigate. For each bribery vote, create a
//!   Treasure token." — GAP: the voting (Council's dilemma) mechanic has no
//!   engine primitive (no vote effect/event), so the trigger payload can't
//!   be modeled.
//! * "While voting, you may vote an additional time." — GAP: same voting
//!   mechanic.
//!
//! Investigate / Council's dilemma / Treasure are not `KeywordAbility`
//! variants; only Flying and Ward({3}) are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tivit, Seller of Secrets");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
