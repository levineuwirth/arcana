//! Latulla, Keldon Overseer — `{3}{R}{R}` 3/3 Legendary Human Spellshaper.
//! `{X}{R}, {T}, Discard two cards:` Latulla deals X damage to any target.
//! WIRED: the `{X}{R}` mana cost fans out and threads the chosen X onto
//! `ctx.x_value`; the "discard two cards" additional cost is `discard_other`
//! with `discard_other_count: 2`; the resolver deals X damage to the chosen
//! target via [`Effect::DealDamage`].

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Latulla, Keldon Overseer");
    let human = reg.interner_mut().intern("Human");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{R}, {T}, Discard two cards: Latulla deals X damage to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{R}").expect("valid cost"),
                    tap: true,
                    discard_other: Some(ObjectFilter::new()),
                    discard_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: x_damage,
            }),
    )
}

fn x_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let damage_target = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: damage_target,
        amount: x,
    }]
}
