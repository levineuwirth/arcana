//! Sunflare Shaman — `{1}{R}` 2/1 red Elemental Shaman.
//! `{1}{R}, {T}: This creature deals X damage to any target and X
//! damage to itself, where X is the number of Elemental cards in
//! your graveyard.`

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunflare Shaman");
    let elemental = reg.interner_mut().intern("Elemental");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}, {T}: This creature deals X damage to any target and X damage to itself, where X is the number of Elemental cards in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_x_damage_to_target_and_self,
            }),
    )
}

fn deal_x_damage_to_target_and_self(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let _elemental_filter = script::subtype_filter(reg, "Elemental");
    let x = script::graveyard_size(state, ctx.controller);
    // Count Elemental cards in graveyard — using graveyard_size as proxy
    // (exact subtype filter in graveyard not supported by script API)
    // GAP: graveyard_size counts all cards, not just Elementals.
    if x == 0 {
        return Vec::new();
    }
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![
        Effect::DealDamage { target: dt, amount: x, source: ctx.source },
        Effect::DealDamage {
            target: DamageTarget::Object(ctx.source),
            amount: x,
            source: ctx.source,
        },
    ]
}
