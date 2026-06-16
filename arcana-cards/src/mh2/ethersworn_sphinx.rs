//! Ethersworn Sphinx — `{7}{W}{U}` 4/4 white-blue Artifact Creature — Sphinx.
//!
//! Affinity for artifacts (this spell costs {1} less to cast for each artifact
//! you control). Flying. Cascade (when you cast this spell, exile from the top
//! of your library until you exile a nonland card with lesser mana value; you
//! may cast it for free; the rest go to the bottom in a random order).
//!
//! Flying is a base keyword. Affinity is a static casting-cost reduction (not a
//! triggered/activated ability) — GAP'd. Cascade is a cast-trigger that fires
//! as the spell is cast (before it resolves / enters); there is no "when you
//! cast this spell" `TriggerCondition` in the demonstrated API to hang the
//! `Effect::Cascade` body on for THIS card's own cast, so it is GAP'd here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ethersworn Sphinx");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Affinity for artifacts is a static casting-cost reduction.
    // GAP: Cascade fires on this spell's own cast; no self-cast TriggerCondition
    //      is available to host the Effect::Cascade body for this card.

    reg.register(CardDefinition::new(name, chars))
}
