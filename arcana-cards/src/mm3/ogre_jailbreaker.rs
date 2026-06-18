//! Ogre Jailbreaker — `{3}{B}` 4/4 Ogre Rogue with Defender.
//! "This creature can attack as though it didn't have defender as long
//! as you control a Gate."
//!
//! Defender is a base keyword. The conditional "can attack as though it
//! didn't have defender" static (gated on controlling a Gate) has no
//! triggered/activated decomposition and no Effect to express it, so it
//! is GAP'd.

// GAP (static): "can attack as though it didn't have defender as long as you
// control a Gate" — a conditional attack-permission continuous ability with
// no expressible form.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ogre Jailbreaker");
    let ogre = reg.interner_mut().intern("Ogre");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
