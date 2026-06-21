//! Cloudsteel Kirin — `{2}{W}` 3/2 Artifact Creature — Equipment Kirin.
//! Flying.
//! GAP: "Equipped creature has flying and 'You can't lose the game and your
//! opponents can't win the game.'" is a continuous static (equipped-creature
//! grant) — not expressible for this class.
//! GAP: Reconfigure {5} is not in the usable keyword surface — omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cloudsteel Kirin");
    let equipment = reg.interner_mut().intern("Equipment");
    let kirin = reg.interner_mut().intern("Kirin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(kirin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
