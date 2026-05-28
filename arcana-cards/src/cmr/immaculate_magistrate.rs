//! Immaculate Magistrate — `{3}{G}` 2/2 Creature — Elf Shaman.
//! `{T}: Put a +1/+1 counter on target creature for each Elf you control.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Immaculate Magistrate");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{T}: Put +1/+1 counters on target creature for each Elf you control.".into(), cost: ActivationCost::tap_only(), target_requirements: vec![TargetRequirement::target_creature()], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: put_counters }))
}

fn put_counters(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let elf_filter = script::subtype_filter(reg, "Elf").controlled_by(arcana_core::targets::ControllerConstraint::You);
    let n = script::count_matching(state, &elf_filter, ctx.controller);
    if n == 0 { return Vec::new(); }
    vec![Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: n }]
}
