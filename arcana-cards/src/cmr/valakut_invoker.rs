//! Valakut Invoker — `{2}{R}` 2/3 red Creature — Human Shaman.
//! {8}: This creature deals 3 damage to any target.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valakut Invoker");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")), colors: ColorSet::red(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(3)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{8}: Deal 3 damage to any target.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{8}").unwrap(), ..ActivationCost::default() }, target_requirements: vec![TargetRequirement::any_target()], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: deal_damage }))
}

fn deal_damage(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    match target {
        TargetChoice::Object(id) => vec![Effect::DealDamage { target: DamageTarget::Object(*id), amount: 3, source: ctx.source }],
        TargetChoice::Player(p) => vec![Effect::DealDamage { target: DamageTarget::Player(*p), amount: 3, source: ctx.source }],
        _ => Vec::new(),
    }
}
