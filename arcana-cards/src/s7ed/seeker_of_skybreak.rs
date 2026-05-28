//! Seeker of Skybreak — `{1}{G}` 2/1 green Creature — Elf.
//! {T}: Untap target creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seeker of Skybreak");
    let elf_sub = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(1)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{T}: Untap target creature.".into(), cost: ActivationCost::tap_only(), target_requirements: vec![TargetRequirement::target_creature()], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: untap_creature }))
}

fn untap_creature(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Untap { target: *id }]
}
