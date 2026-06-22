//! Rix Maadi Guildmage — `{B}{R}` 2/2 Human Shaman.
//! `{B}{R}: Target blocking creature gets -1/-1 until end of turn.`
//! `{B}{R}: Target player who lost life this turn loses 1 life.`

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rix Maadi Guildmage");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{R}: Target blocking creature gets -1/-1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                // Partial: oracle restricts to a BLOCKING creature; the documented
                // TargetFilter surface has no blocking-only constraint.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: weaken_blocker,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{R}: Target player who lost life this turn loses 1 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                // Partial: oracle restricts to a player WHO LOST LIFE THIS TURN; the
                // documented TargetFilter surface has no such player constraint.
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: drain_one,
            }),
    )
}

fn weaken_blocker(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn drain_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::LoseLife {
        player: *p,
        amount: 1,
    }]
}
