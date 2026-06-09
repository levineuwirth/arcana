//! Rockcaster Platoon — `{5}{W}{W}` 5/7 Rhino Soldier.
//! `{4}{G}: This creature deals 2 damage to each creature with flying and each player.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rockcaster Platoon");
    let rhino = reg.interner_mut().intern("Rhino");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{G}: This creature deals 2 damage to each creature with flying and each player.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_flyers_and_players,
            }),
    )
}

fn damage_flyers_and_players(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
        ctx.controller,
    );
    let mut effects: Vec<Effect> = creature_ids
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 2,
        })
        .collect();
    let players = script::all_players(state);
    for p in players {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 2,
        });
    }
    effects
}
