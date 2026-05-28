//! Embodiment of Spring — `{U}` 0/3 Creature — Elemental.
//! `{1}{G}, {T}, Sacrifice this creature: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Embodiment of Spring");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(0)), toughness: Some(PtValue::Fixed(3)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{1}{G}, {T}, Sacrifice this creature: Search your library for a basic land, put it onto the battlefield tapped.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{1}{G}").unwrap(), tap: true, sacrifice: true, ..ActivationCost::default() }, target_requirements: Vec::new(), is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: tutor_land }))
}

fn tutor_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield { player: ctx.controller, filter: ObjectFilter::new().with_types(TypeLine::LAND.into()).with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)), tapped: true }]
}
