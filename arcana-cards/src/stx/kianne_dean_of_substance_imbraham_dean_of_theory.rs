//! Kianne, Dean of Substance // Imbraham, Dean of Theory
//!
//! Front face: Legendary Creature — Elf Druid {2}{G} 2/2
//! {T}: Exile the top card of your library. If it's a land card, put it into
//! your hand. Otherwise, put a study counter on it.
//! {4}{G}: Create a 0/0 green and blue Fractal creature token. Put a +1/+1
//! counter on it for each different mana value among nonland cards you own in
//! exile with study counters on them.
//!
//! Back face: Legendary Creature — Bird Wizard (MDFC back, cast separately)
//! Flying
//! {X}{U}{U}, {T}: Exile the top X cards of your library and put a study counter
//! on each of them. Then you may put a card you own in exile with a study counter
//! on it into your hand.
//!
//! GAP: Kianne's {T} ability (exile top / land-to-hand / else study counter) is not
//! expressible as a single Effect variant — the "if land put to hand, else counter"
//! branch requires conditional zone routing not in the catalog.
//! GAP: Kianne's {4}{G} ability — counting distinct mana values among exile-with-study
//! is not computable with script::* helpers.
//! GAP: Imbraham's {X}{U}{U},{T} ability — back-face activated ability with X cost
//! and exile-with-study logic; back-face activated abilities require face_gate and
//! the exile-with-study routing is not in the catalog.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kianne, Dean of Substance");
    let elf_sub = reg.interner_mut().intern("Elf");
    let druid_sub = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf_sub);
    subtypes.0.insert(druid_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Imbraham, Dean of Theory — Legendary Bird Wizard, Flying
    let back_name = reg.interner_mut().intern("Imbraham, Dean of Theory");
    let bird_sub = reg.interner_mut().intern("Bird");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(bird_sub);
    back_subtypes.0.insert(wizard_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{X}{U}{U}").expect("valid cost")),
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back),
    )
}
