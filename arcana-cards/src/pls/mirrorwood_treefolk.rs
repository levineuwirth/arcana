//! Mirrorwood Treefolk — `{3}{G}` 2/4 Creature — Treefolk.
//! `{2}{R}{W}: The next time damage would be dealt to this creature this turn, that damage is dealt to any target instead.`
//! GAP: "redirect damage to any target" — no Effect for damage redirection.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirrorwood Treefolk");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(4)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{2}{R}{W}: Redirect next damage to any target.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{2}{R}{W}").unwrap(), ..ActivationCost::default() }, target_requirements: vec![TargetRequirement::any_target()], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: redirect }))
}

fn redirect(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no Effect for damage redirection
    Vec::new()
}
