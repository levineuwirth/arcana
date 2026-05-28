//! Glintwing Invoker — `{4}{U}` 3/3 Creature — Human Wizard Mutant.
//! `{7}{U}: This creature gets +3/+3 and gains flying until end of turn.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glintwing Invoker");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(mutant);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(3)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{7}{U}: This creature gets +3/+3 and gains flying until end of turn.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{7}{U}").unwrap(), ..ActivationCost::default() }, target_requirements: Vec::new(), is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: pump_and_fly }))
}

fn pump_and_fly(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump { target: ctx.source, power: 3, toughness: 3, duration: Duration::EndOfTurn, keywords: vec![KeywordAbility::Flying] }]
}
