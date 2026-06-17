//! Adric, Mathematical Genius — `{1}{U}` 1/1 Legendary Human Artificer.
//! Two activated abilities operating on activated/triggered abilities on the
//! stack — neither is expressible (GAP). Doctor's companion is not a usable
//! keyword.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adric, Mathematical Genius");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    // GAP: "{2}{U}, {T}: Copy target activated or triggered ability you
    // control" — no Effect copies an ability on the stack and no TargetFilter
    // targets an ability.
    // GAP: "Ultimate Sacrifice — {1}{U}, Sacrifice Adric: Counter target
    // activated or triggered ability" — no Effect counters an ability and no
    // TargetFilter targets one.
    // GAP: keyword — Doctor's companion is not in the usable keyword surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
