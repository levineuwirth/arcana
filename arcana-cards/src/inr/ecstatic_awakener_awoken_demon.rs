//! Ecstatic Awakener // Awoken Demon — `{B}` Human Wizard creature 1/1 (front) /
//! Demon creature (back). Transform TDFC.
//!
//! Front face:
//!   {2}{B}, Sacrifice another creature: Draw a card, then transform this creature.
//!   Activate only once each turn.
//!
//! Back face (Awoken Demon):
//!   (No oracle text — bare 4/4 black Demon.)
//!
//! # Notes
//! - Activation cost: {2}{B} + sacrifice another creature → `mana_cost` + `sacrifice_other`.
//! - "Activate only once each turn" — no per-turn activation gate in ActivationCost;
//!   the engine does not enforce this limit. GAP: once-per-turn restriction not modeled.
//! - Back face P/T: 4/4 per oracle (inferred from Awoken Demon).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ecstatic Awakener");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Awoken Demon — 4/4 black Demon
    let back_name = reg.interner_mut().intern("Awoken Demon");
    let demon_sub = reg.interner_mut().intern("Demon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(demon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {2}{B}, Sacrifice another creature: Draw a card, then transform.
            // GAP: "activate only once each turn" restriction not modeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, Sacrifice another creature: Draw a card, then transform this creature. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: awakener_activate,
            }),
    )
}

fn awakener_activate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Transform { target: ctx.source },
    ]
}
