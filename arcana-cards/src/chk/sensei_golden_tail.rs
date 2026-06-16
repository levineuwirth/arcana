//! Sensei Golden-Tail — `{1}{W}` 2/1 Legendary Fox Samurai.
//! Bushido 1.
//! `{1}{W}, {T}: Put a training counter on target creature. That creature
//! gains bushido 1 and becomes a Samurai in addition to its other creature
//! types. Activate only as a sorcery.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sensei Golden-Tail");
    let fox = reg.interner_mut().intern("Fox");
    let samurai = reg.interner_mut().intern("Samurai");
    let training = reg.interner_mut().intern("training");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bushido(1)],
        ..Default::default()
    };

    let _ = training;

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{W}, {T}: Put a training counter on target creature. That creature gains \
                   bushido 1 and becomes a Samurai in addition to its other creature types. \
                   Activate only as a sorcery."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: train_target,
        }),
    )
}

fn train_target(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let training = reg
        .interner()
        .lookup("training")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    // GAP: "gains bushido 1 and becomes a Samurai in addition to its other
    // creature types" — permanent keyword/subtype grants are not expressible
    // with the documented Effect surface (GrantKeyword durations are EOT /
    // WhileSourceOnBattlefield; no add-subtype Effect).
    vec![Effect::AddCounters {
        target: *id,
        kind: training,
        count: 1,
    }]
}
