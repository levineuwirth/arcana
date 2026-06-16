//! Avatar Aang // Aang, Master of Elements — `{R}{G}{W}{U}` Legendary Creature
//! — Human Avatar Ally 4/4, transforms to Legendary Creature — Avatar Ally.
//!
//! Front face: Flying, firebending 2.
//! Whenever you waterbend, earthbend, firebend, or airbend, draw a card.
//! Then if you've done all four this turn, transform Avatar Aang.
//! GAP: "firebending 2" keyword not modeled (custom mechanic, not in engine).
//! GAP: "waterbend/earthbend/firebend/airbend" events not modeled in engine
//!   (custom mechanic); draw-a-card trigger and "done all four" transform
//!   condition not expressible.
//!
//! Back face: Aang, Master of Elements — Flying.
//! Spells you cast cost {W}{U}{B}{R}{G} less to cast.
//! GAP: "spells cost less" static continuous effect not expressible via
//!   current anthem-only continuous effect builder.
//! At the beginning of each upkeep, you may transform Aang, Master of Elements.
//! If you do, you gain 4 life, draw four cards, put four +1/+1 counters on him,
//! and he deals 4 damage to each opponent.
//! GAP: back-face-only triggered ability (upkeep transform + effects) not
//!   auto-installed on transform.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar Aang");
    let human_sub = reg.interner_mut().intern("Human");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let ally_sub = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(avatar_sub);
    subtypes.0.insert(ally_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Aang, Master of Elements");
    let back_avatar_sub = reg.interner_mut().intern("Avatar");
    let back_ally_sub = reg.interner_mut().intern("Ally");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_avatar_sub);
    back_subtypes.0.insert(back_ally_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green() | ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: front-face "waterbend/earthbend/firebend/airbend" triggered
        //   abilities not modeled (custom mechanic events not in engine).
        // GAP: back-face upkeep triggered ability (optional transform + gain 4
        //   life, draw 4, add 4 counters, deal 4 to each opponent) not
        //   auto-installed on transform.
        // GAP: back-face "spells cost {W}{U}{B}{R}{G} less" continuous effect
        //   not expressible via current engine.
    )
}
