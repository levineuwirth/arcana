//! Skirk Ridge Exhumer — `{1}{B}` 1/1 Zombie Spellshaper.
//! `{B}, {T}, Discard a card:` Create a 1/1 black Zombie Goblin creature
//! token named Festering Goblin. It has "When this token dies, target
//! creature gets -1/-1 until end of turn."
//! GAP: Token with a triggered ability (dies-trigger) not expressible in
//! TokenDefinition.abilities; emitting bare token only.

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
    let name = reg.interner_mut().intern("Skirk Ridge Exhumer");
    let zombie = reg.interner_mut().intern("Zombie");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let _festering = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}, Discard a card: Create a 1/1 black Zombie Goblin creature token named Festering Goblin.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    tap: true,
                    // GAP: "Discard a card" (any card) not in ActivationCost.
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_festering_goblin,
            }),
    )
}

fn create_festering_goblin(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin")
        .expect("Goblin interned during register()");
    let zombie = reg.interner().lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(zombie);
    token_subtypes.0.insert(goblin);
    // GAP: token's dies-trigger ("-1/-1 to target creature") not wired.
    let token = TokenDefinition {
        name: goblin,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
