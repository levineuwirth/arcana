//! Eastern Paladin — `{2}{B}{B}` 3/3 black Creature — Phyrexian Zombie Knight.
//! {B}{B}, {T}: Destroy target green creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eastern Paladin");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let zombie_sub = reg.interner_mut().intern("Zombie");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian_sub);
    subtypes.0.insert(zombie_sub);
    subtypes.0.insert(knight_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(3)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{B}{B}, {T}: Destroy target green creature.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{B}{B}").unwrap(), tap: true, ..ActivationCost::default() }, target_requirements: vec![TargetRequirement { filter: TargetFilter::Permanent(ObjectFilter::creature().with_colors(ColorSet::green())), count: TargetCount::Exactly(1), controller: None }], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: destroy_green }))
}

fn destroy_green(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
