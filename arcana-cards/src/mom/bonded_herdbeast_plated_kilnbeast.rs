//! Bonded Herdbeast // Plated Kilnbeast — {4}{G} 4/5 green Beast creature.
//!
//! Front face (Bonded Herdbeast):
//!   {4}{R/P}: Transform this creature. Activate only as a sorcery.
//!   ({R/P} can be paid with either {R} or 2 life.)
//!   GAP: {R/P} hybrid/Phyrexian mana not fully supported in ManaCost::parse;
//!   modeled as {4}{R} (the mana-payment alternative). Sorcery-speed restriction
//!   is modeled via is_instant_speed: false.
//!
//! Back face (Plated Kilnbeast):
//!   Menace (can't be blocked except by two or more creatures.)
//!   GAP: Back-face-only keyword Menace not auto-installed on transform.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bonded Herdbeast");
    let beast_sub = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Plated Kilnbeast");
    let back_phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_beast_sub = reg.interner_mut().intern("Beast");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_phyrexian_sub);
    back_subtypes.0.insert(back_beast_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            // Back face gains Phyrexian Beast — stays green per oracle
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: Menace is a back-face-only keyword; not auto-installed on transform.
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {4}{R/P}: Transform this creature. Activate only as a sorcery.
            // GAP: {R/P} Phyrexian mana modeled as {4}{R} (mana-payment alternative only).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{R/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false, // sorcery speed
                face_gate: Some(0), // front face only
                effect: do_transform,
            })
    )
}

fn do_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
