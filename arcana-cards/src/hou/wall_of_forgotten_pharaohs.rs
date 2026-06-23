//! Wall of Forgotten Pharaohs — `{2}` 0/4 Artifact Creature — Wall.
//! Defender.
//! {T}: This creature deals 1 damage to target player or planeswalker.
//! Activate only if you control a Desert or there is a Desert card in your
//! graveyard.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Forgotten Pharaohs");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: This creature deals 1 damage to target player or planeswalker. Activate only if you control a Desert or there is a Desert card in your graveyard.".into(),
            cost: ActivationCost {
                tap: true,
                activation_condition: Some(controls_or_gy_desert),
                ..ActivationCost::default()
            },
            // GAP (fidelity): "target player or planeswalker" — modeled as
            // target player; the planeswalker option isn't a TargetFilter form.
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: ping_player,
        }),
    )
}

use arcana_core::mana::ManaCost;

fn controls_or_gy_desert(
    state: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    conditions::you_control_subtype(state, reg, you, "Desert")
        || conditions::graveyard_has_subtype(state, reg, you, "Desert")
}

fn ping_player(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Player(*p),
        amount: 1,
    }]
}
