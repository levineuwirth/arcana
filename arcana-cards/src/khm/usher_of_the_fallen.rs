//! Usher of the Fallen — `{W}` 2/1 white Spirit Warrior. Boast — `{1}{W}`: Create
//! a 1/1 white Human Warrior creature token. (Activate only if this creature attacked
//! this turn and only once each turn.)
//!
//! The "attacked this turn" half of Boast is enforced via
//! `ActivationCost::activation_condition` + `conditions::source_attacked_this_turn`;
//! "only once each turn" via `once_per_turn`.

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
    let name = reg.interner_mut().intern("Usher of the Fallen");
    let spirit = reg.interner_mut().intern("Spirit");
    let warrior = reg.interner_mut().intern("Warrior");
    // Pre-intern token subtypes for resolve-time lookup.
    let _human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Boast — {1}{W}: Create a 1/1 white Human Warrior creature token. (Activate only if this creature attacked this turn and only once each turn.)".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").unwrap(),
                    // Boast: only if this creature attacked this turn.
                    activation_condition: Some(|s, src, _you, _reg| {
                        arcana_core::conditions::source_attacked_this_turn(s, src)
                    }),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: create_human_warrior_token,
            }),
    )
}

fn create_human_warrior_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human")
        .expect("Human interned during register()");
    let warrior = reg.interner().lookup("Warrior")
        .expect("Warrior interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(human);
    token_subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
