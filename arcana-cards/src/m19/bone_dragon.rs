//! Bone Dragon — `{3}{B}{B}` 5/4 Creature — Dragon Skeleton.
//!
//! Oracle:
//! * Flying.
//! * `{3}{B}{B}, Exile seven other cards from your graveyard: Return this
//!   card from your graveyard to the battlefield tapped.`
//!   GAP: the activation cost "exile seven other cards from your graveyard"
//!   has no expressible ActivationCost field (no exile-from-graveyard cost
//!   variant), and there is no self-return-from-graveyard-tapped effect on
//!   the supported surface. Whole ability omitted rather than emit it with
//!   a missing cost.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bone Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
