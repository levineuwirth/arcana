//! Tayam, Luminous Enigma — `{1}{W}{B}{G}` 3/3 Legendary Nightmare Beast.
//! "Each other creature you control enters with an additional vigilance counter
//!  on it."
//! "{3}, Remove three counters from among creatures you control: Mill three
//!  cards, then return a permanent card with mana value 3 or less from your
//!  graveyard to the battlefield."
//!
//! Both abilities are GAP'd:
//!  - The "each other creature you control enters with a vigilance counter"
//!    replacement static has no expressible representation.
//!  - The activated ability's cost ("remove three counters from AMONG creatures
//!    you control") is not expressible — the only counter-removal cost field is
//!    remove_self_counter (this creature only), not a spread across other
//!    permanents. Without a faithful cost the whole ability is omitted.
//! "Mill" appears as a Scryfall keyword tag but is not a real KeywordAbility.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tayam, Luminous Enigma");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static — "each other creature you control enters with an additional
    // vigilance counter on it." No expressible enters-with-counter replacement
    // for other permanents.
    // GAP: activated ability — "{3}, Remove three counters from among creatures
    // you control: ..." The remove-counters-from-among-others cost is not
    // expressible (only remove_self_counter exists); the whole ability is
    // omitted rather than emit it with a wrong cost.
    reg.register(CardDefinition::new(name, chars))
}
