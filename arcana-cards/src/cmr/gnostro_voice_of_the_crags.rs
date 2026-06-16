//! Gnostro, Voice of the Crags — `{1}{U}{R}{W}` 3/3 Legendary Chimera.
//! "{T}: Choose one. X is the number of spells you've cast this turn.
//!  • Scry X.  • Gnostro deals X damage to target creature.
//!  • You gain X life."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnostro, Voice of the Crags");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // "Scry" parsed by Scryfall here is the choose-one mode text, not a
        // standalone keyword — no keyword line.
        keywords: vec![],
        ..Default::default()
    };
    // GAP (activated ability): "{T}: Choose one ..." — modal dispatch
    // (ModalSpec / with_mode_effects) exists only for SPELL abilities; there is
    // no modal path on ActivatedAbilityDef. Emitting bones only rather than
    // forcing a single (wrong) mode.
    reg.register(CardDefinition::new(name, chars))
}
