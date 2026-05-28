//! Jhoira, Ageless Innovator — `{U}{R}` 2/3 Legendary Human Artificer.
//! `{T}:` Put two ingenuity counters on Jhoira, then you may put an artifact
//! card with mana value X or less from your hand onto the battlefield, where
//! X is the number of ingenuity counters on Jhoira.
//! GAP: Custom "ingenuity" counter (non-standard CounterKind), X-scaling
//! based on own counters, and "put from hand to battlefield" not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jhoira, Ageless Innovator");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Put two ingenuity counters on Jhoira, then you may put an artifact card with mana value X or less from your hand onto the battlefield, where X is the number of ingenuity counters on Jhoira.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: innovator_ability,
            }),
    )
}

fn innovator_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "ingenuity" counter (custom type), X based on own counters, and
    // "put artifact from hand to battlefield" not expressible.
    Vec::new()
}
