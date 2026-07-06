//! Runebound Wolf — `{1}{R}` 2/2 red Wolf.
//! "{3}{R}, {T}: This creature deals damage equal to the number of Wolves and Werewolves
//! you control to target opponent."

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
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Runebound Wolf");
    let wolf_sub = reg.interner_mut().intern("Wolf");
    let _werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}, {T}: This creature deals damage equal to the number of Wolves and Werewolves you control to target opponent.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_wolf_count_damage,
            }),
    )
}

fn deal_wolf_count_damage(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf_filter = ObjectFilter::creature()
        .with_subtypes_any(vec![
            reg.interner().lookup("Wolf").expect("Wolf interned"),
            reg.interner().lookup("Werewolf").expect("Werewolf interned"),
        ])
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &wolf_filter, ctx.controller);
    if n == 0 { return Vec::new(); }
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*p),
        amount: n,
        source: ctx.source,
    }]
}
