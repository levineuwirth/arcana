//! A-Sunbathing Rootwalla — `{1}{G}` 2/2 Lizard.
//! Domain — `{1}{G}:` Until end of turn, this creature gets +1/+1 for each
//! basic land type among lands you control. Activate only once each turn.
//! GAP: Domain scaling (count of distinct basic land types) not available
//! via script helpers; emitting Vec::new().

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
    let name = reg.interner_mut().intern("A-Sunbathing Rootwalla");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Domain — {1}{G}: Until end of turn, this creature gets +1/+1 for each basic land type among lands you control. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").unwrap(),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: domain_pump,
            }),
    )
}

fn domain_pump(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Domain — "for each basic land type among lands you control" requires
    // a script helper to count distinct basic land types on the battlefield;
    // no such helper exists in the current API.
    Vec::new()
}
