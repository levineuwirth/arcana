//! Huskburster Swarm — `{7}{B}` 6/6 Elemental Insect.
//! "This spell costs {1} less to cast for each creature card you own in
//!  exile and in your graveyard. Menace, deathtouch."
//!
//! Menace and deathtouch are keywords. The cost-reduction static
//! (cost-altering continuous ability) is not expressible as a
//! triggered/activated ability and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: static — "This spell costs {1} less to cast for each creature card you
// own in exile and in your graveyard" is a cost-reduction continuous ability,
// not a triggered/activated ability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huskburster Swarm");
    let elemental = reg.interner_mut().intern("Elemental");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
