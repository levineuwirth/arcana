//! Tezzeret's Simulacrum — `{3}` 2/3 Artifact Creature — Golem.
//! `{T}: Target opponent loses 1 life. If you control a Tezzeret planeswalker, that player loses 3 life instead.`
//! GAP: "if you control a Tezzeret planeswalker, 3 life instead" — conditional based on controlling a specific planeswalker type not expressible; emitting 1 life loss unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret's Simulacrum");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target opponent loses 1 life. If you control a Tezzeret planeswalker, that player loses 3 life instead.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_opponent,
            }),
    )
}

fn ping_opponent(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you control a Tezzeret planeswalker, loses 3 instead" — conditional not expressible; emitting 1 life loss
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::LoseLife { player: *p, amount: 1 }]
}
