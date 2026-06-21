//! Dragonkin Berserker — `{1}{R}` 2/2 Human Berserker.
//! First strike.
//! Boast abilities you activate cost {1} less for each Dragon you control.
//! (static cost reduction — GAP)
//! Boast — {4}{R}: Create a 5/5 red Dragon creature token with flying.
//! (Activate only if this creature attacked this turn and only once each turn.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonkin Berserker");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: static — "Boast abilities you activate cost {1} less to activate
    // for each Dragon you control." No dynamic cost-reduction primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Boast — {4}{R}: Create a 5/5 red Dragon creature token with flying.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{R}").unwrap(),
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
            is_instant_speed: false,
            face_gate: None,
            effect: boast_make_dragon,
        }),
    )
}

fn boast_make_dragon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
