//! Detective's Phoenix — `{2}{R}` 2/2 red Enchantment Creature — Phoenix.
//! "Bestow—{R}, Collect evidence 6. Flying, haste. Enchanted creature gets
//! +2/+2 and has flying and haste. You may cast this card from your
//! graveyard using its bestow ability."
//!
//! Only Flying and Haste are expressible. Bestow (alternative-cost cast
//! that makes the card an Aura), Collect evidence, the bestowed aura's
//! +2/+2-and-keywords static, and the cast-from-graveyard permission are
//! all unexpressible — recorded as GAPs.
//!
//! GAP: Bestow—{R}, Collect evidence 6 (alternative cast / aura mechanic).
//! GAP: Collect evidence (not in the supported keyword surface).
//! GAP: static — "Enchanted creature gets +2/+2 and has flying and haste."
//! GAP: static — "You may cast this card from your graveyard using bestow."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Detective's Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
