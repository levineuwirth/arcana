//! Necropolis Fiend — `{7}{B}{B}` 4/5 Creature — Demon.
//!
//! Oracle:
//! * Delve.
//! * Flying.
//! * {X}, {T}, Exile X cards from your graveyard: Target creature gets -X/-X
//!   until end of turn.
//!
//! Decomposition: only the Flying keyword is expressible.
//!
//! GAP: Delve (cast-time cost reduction by exiling graveyard cards) is not in
//!      the supported keyword surface — omitted.
//! GAP: the "{X}, {T}, Exile X cards from your graveyard: target creature gets
//!      -X/-X" activated ability has no expressible cost — there is no
//!      "exile X cards from graveyard" activation-cost field nor a generic X
//!      mana cost (cf. Taigam, Sidisi's Hand). Omitted whole rather than
//!      emitting an ungated -X/-X effect.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necropolis Fiend");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
