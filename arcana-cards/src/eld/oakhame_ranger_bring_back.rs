//! Oakhame Ranger // Bring Back — `{G/W}{G/W}{G/W}{G/W}` // `{G/W}{G/W}{G/W}{G/W}` green/white Adventure creature.
//! Creature: 2/2 Elf Knight Ranger. "{T}: Creatures you control get +1/+1 until end of turn."
//! Adventure (Bring Back — Sorcery): Create two 1/1 white Human creature tokens.
//! GAP: "{T}: creatures get +1/+1" — activated tap ability that pumps all creatures not fully modeled without script access.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oakhame Ranger");
    let adv_name = reg.interner_mut().intern("Bring Back");
    let elf_sub = reg.interner_mut().intern("Elf");
    let knight_sub = reg.interner_mut().intern("Knight");
    let ranger_sub = reg.interner_mut().intern("Ranger");
    let _human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf_sub);
    subtypes.0.insert(knight_sub);
    subtypes.0.insert(ranger_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{G/W}{G/W}{G/W}{G/W}").expect("valid cost")), colors: ColorSet::green() | ColorSet::white(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)), ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{G/W}{G/W}{G/W}{G/W}").expect("valid cost")), colors: ColorSet::green() | ColorSet::white(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Create two 1/1 white Human creature tokens.".into(), target_requirements: vec![], modal: None, effect: bring_back_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_activated_ability(ActivatedAbilityDef { text: "{T}: Creatures you control get +1/+1 until end of turn.".into(), cost: ActivationCost { tap: true, ..ActivationCost::default() }, target_requirements: vec![], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: tap_pump })
            .with_adventure(adventure),
    )
}

fn tap_pump(state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature().controlled_by(ControllerConstraint::You), ctx.controller);
    ids.into_iter().map(|id| Effect::Pump { target: id, power: 1, toughness: 1, duration: Duration::EndOfTurn, keywords: vec![] }).collect()
}

fn bring_back_resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").expect("interned at register");
    let mut ts = SubtypeSet::default();
    ts.0.insert(human);
    let token = TokenDefinition { name: human, colors: ColorSet::white(), types: TypeLine::CREATURE.into(), subtypes: ts, power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)), keywords: vec![], abilities: vec![] };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
