//! Sorin, Vengeful Bloodlord — `{2}{W}{B}` Legendary Planeswalker — Sorin, starting loyalty 4.
//!
//! Static: "During your turn, creatures and planeswalkers you control have
//!   lifelink." — a conditional anthem-style keyword grant; not a loyalty ability
//!   and not expressible from the demonstrated surface. GAP (static, not modeled).
//! +2: Sorin deals 1 damage to target player or planeswalker. Modeled with
//!   `Effect::DealDamage` over an any-target choice (player or planeswalker).
//! −X: Return target creature card with mana value X from your graveyard to the
//!   battlefield; it becomes a Vampire. GAP: a dynamic −X loyalty cost is not
//!   expressible (`remove_self_counter` is a fixed u32). Ability omitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin, Vengeful Bloodlord");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Sorin deals 1 damage to target player or planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_damage,
            }),
        // GAP: "-X: Return target creature card with mana value X from your
        //       graveyard to the battlefield. That creature is a Vampire in
        //       addition to its other types." — dynamic-X loyalty cost is not
        //       expressible; ability omitted.
    )
}

fn plus_two_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { source: ctx.source, target: dt, amount: 1 }]
}
