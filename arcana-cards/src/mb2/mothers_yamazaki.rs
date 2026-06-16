//! Mothers Yamazaki — `{2}{R}{W}` 2/2 Legendary Human Samurai.
//! Partner with itself.
//! As long as you control exactly two permanents named Mothers Yamazaki, the
//! "legend rule" doesn't apply to them, and Samurai you control get +2/+2 and
//! have vigilance and haste.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mothers Yamazaki");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Partner with" / "Partner" are not KeywordAbility variants.
        ..Default::default()
    };
    // GAP: "Partner with itself" ETB (target player may tutor this card to hand,
    // then shuffle) — the partner-with mechanic is not modeled in this surface.
    // GAP static: "as long as you control exactly two permanents named Mothers
    // Yamazaki, the legend rule doesn't apply ... and Samurai you control get
    // +2/+2 and have vigilance and haste" — a conditional continuous static, not
    // a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
