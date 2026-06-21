//! Elesh Norn, Mother of Machines — `{4}{W}` 4/7 Legendary Phyrexian Praetor
//! with Vigilance.
//!
//! Oracle:
//! * Vigilance.
//! * If a permanent entering causes a triggered ability of a permanent you
//!   control to trigger, that ability triggers an additional time.
//! * Permanents entering don't cause abilities of permanents your opponents
//!   control to trigger.
//!
//! Vigilance is a base characteristic. Both remaining lines are static
//! replacement effects that rewrite how ETB triggers fire — there is no
//! triggered/activated ability or Effect that expresses either, so both are
//! GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elesh Norn, Mother of Machines");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let praetor = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(praetor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "If a permanent entering causes a triggered ability of a permanent
    // you control to trigger, that ability triggers an additional time" —
    // static trigger-doubling replacement; no expressible primitive.
    // GAP: "Permanents entering don't cause abilities of permanents your
    // opponents control to trigger" — static trigger-suppression replacement;
    // no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
