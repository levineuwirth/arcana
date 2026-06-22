//! Phyrexian Colossus — `{7}` 8/8 Artifact Creature — Phyrexian Golem
//! (colorless).
//! "This creature doesn't untap during your untap step.
//!  Pay 8 life: Untap this creature.
//!  This creature can't be blocked except by three or more creatures."
//!
//! Only the "Pay 8 life: Untap" activated ability is expressible.
//! GAP: "doesn't untap during your untap step" is a static untap
//!      restriction — no Effect / field expresses it.
//! GAP: "can't be blocked except by three or more creatures" is a static
//!      block restriction with a count clause — not expressible (the
//!      CantBeBlocked Effect is unconditional, not menace-with-count).

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
    let name = reg.interner_mut().intern("Phyrexian Colossus");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Pay 8 life: Untap this creature.".into(),
                cost: ActivationCost {
                    life: 8,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_self,
            }),
    )
}

fn untap_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
