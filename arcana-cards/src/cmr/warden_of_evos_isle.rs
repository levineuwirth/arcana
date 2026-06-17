//! Warden of Evos Isle — `{2}{U}` 2/2 blue Bird Wizard.
//! Flying. "Creature spells with flying you cast cost {1} less to cast."
//!
//! Only the keyword line is expressible; the cost-reduction static is a
//! continuous spell-cost modifier with no triggered/activated form here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warden of Evos Isle");
    let bird = reg.interner_mut().intern("Bird");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static cost-reduction "creature spells with flying you cast cost {1}
    // less" is not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
