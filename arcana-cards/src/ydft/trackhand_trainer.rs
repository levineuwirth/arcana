//! Trackhand Trainer — `{U}` 1/1 Merfolk.
//! "{4}{U}: Draw a card.
//!  Exhaust — {U}: Conjure a card named Training Grounds onto the
//!  battlefield."
//!
//! The first activated ability is expressible. The second is an Exhaust
//! ability (activate-only-once, no engine field for it) whose payload
//! is Conjure (Arena-only, not modeled) — its effect is GAP'd; the
//! Exhaust restriction itself is also a gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trackhand Trainer");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{U}: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_a_card,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: Exhaust (activate only once, ever) has no ActivationCost field.
                text: "Exhaust — {U}: Conjure a card named Training Grounds onto the battlefield.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: conjure_training_grounds,
            }),
    )
}

fn draw_a_card(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn conjure_training_grounds(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name lookup in Effect::execute)
    Vec::new()
}
