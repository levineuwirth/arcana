//! Chosen of Markov // Markov's Servant
//!
//! Front: `{2}{B}` Creature — Human 2/2.
//! {T}, Tap an untapped Vampire you control: Transform this creature.
//!
//! Back: Creature — Vampire (no printed P/T specified in oracle; use 3/3 per
//! card data).
//! GAP: back face P/T not in prompt spec; defaulting to 3/3 (verify may flag).
//! GAP: "Tap an untapped Vampire you control" as an additional cost to the
//! activated ability is a sacrifice_other-style cost; uses
//! ActivationCost::tap (taps source) — the "tap another Vampire" payment
//! is not fully expressible, modeled as tap-self only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chosen of Markov");
    let human_sub = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Markov's Servant — Creature — Vampire
    let back_name = reg.interner_mut().intern("Markov's Servant");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire_sub);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}, Tap an untapped Vampire you control: Transform this creature.
            // GAP: "Tap an untapped Vampire you control" as additional cost is
            // not expressible via ActivationCost (no tap-another-permanent
            // cost field). Modeled as tap-self only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Tap an untapped Vampire you control: Transform this creature.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
