//! Fearless Liberator — `{1}{R}` 2/1 red Dwarf Berserker.
//! "Boast — {2}{R}: Create a 2/1 red Dwarf Berserker creature token.
//! (Activate only if this creature attacked this turn and only once each turn.)"
//!
//! Boast is not a keyword in the engine; modeled as a regular activated
//! ability with mana cost. The "attacked this turn" half is enforced via
//! `ActivationCost::activation_condition` + `conditions::source_attacked_this_turn`.
//! GAP: "only once each turn" — no per-turn activation counter on ActivationCost.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fearless Liberator");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Boast — {2}{R}: Create a 2/1 red Dwarf Berserker creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").unwrap(),
                    // Boast: only if this creature attacked this turn.
                    activation_condition: Some(|s, src, _you, _reg| {
                        arcana_core::conditions::source_attacked_this_turn(s, src)
                    }),
                    // GAP: "only once each turn" not expressible.
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_dwarf_token,
            }),
    )
}

fn create_dwarf_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dwarf = reg.interner().lookup("Dwarf").expect("Dwarf interned during register()");
    let berserker = reg.interner().lookup("Berserker").expect("Berserker interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(berserker);
    let token = TokenDefinition {
        name: dwarf,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
