//! Seraph of New Capenna // Seraph of New Phyrexia
//!
//! Front face: {2}{W} Creature — Angel Soldier 2/2, Flying. Activated ability:
//! {4}{B/P}: Transform this creature. Sorcery speed.
//! ({B/P} can be paid with either {B} or 2 life — GAP: hybrid Phyrexian mana not
//! expressible as a single ManaCost::parse string; the activated ability is omitted
//! and noted below.)
//!
//! Back face: Creature — Phyrexian Angel, Flying. Whenever this creature attacks,
//! you may sacrifice another creature or artifact. If you do, this creature gets
//! +2/+1 until end of turn.
//!
//! GAP: {B/P} hybrid-Phyrexian activation cost not expressible; the transform
//! activated ability (front → back) is omitted.
//! GAP: "you may sacrifice another creature or artifact" — OptionalPaymentKind has
//! no Sacrifice variant; the conditional pump on the back face is also omitted.
//! GAP: back-face-only triggered ability not modeled (attack trigger granting +2/+1
//! conditional on sacrifice).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seraph of New Capenna");
    let angel_sub = reg.interner_mut().intern("Angel");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Seraph of New Phyrexia");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let angel_back_sub = reg.interner_mut().intern("Angel");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(angel_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: {4}{B/P} transform activation not modeled (hybrid-Phyrexian cost unsupported).
    // GAP: back-face attack trigger (sacrifice another creature or artifact for +2/+1) not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
