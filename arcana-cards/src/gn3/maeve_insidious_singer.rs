//! Maeve, Insidious Singer — `{2}{U}{U}` 3/4 Legendary Creature — Siren.
//! `{2}{U}: Goad target creature. Whenever that creature attacks one of your opponents
//!  this turn, you draw a card.`
//! NOTE: The "whenever that creature attacks one of your opponents this turn" rider requires
//! a delayed/conditional trigger tied to a specific goaded creature — not expressible in the
//! current trigger model. Emitting the Goad effect; the draw-card-when-it-attacks rider is a GAP.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maeve, Insidious Singer");
    let siren = reg.interner_mut().intern("Siren");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}: Goad target creature. Whenever that creature attacks one of your opponents this turn, you draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: goad_creature,
            }),
    )
}

fn goad_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "whenever that creature attacks one of your opponents this turn, draw a card" rider
    //      requires a delayed per-object trigger — not modeled in the current engine
    vec![Effect::Goad {
        target: *id,
        goader: ctx.controller,
        duration: Duration::EndOfTurn,
    }]
}
