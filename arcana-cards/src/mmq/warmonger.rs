//! Warmonger — `{3}{R}` 3/3 red Minotaur Monger.
//! "{2}: This creature deals 1 damage to each creature without flying and each player.
//! Any player may activate this ability."
//! GAP: "each creature without flying" — no "without flying" filter in ObjectFilter.
//! GAP: "Any player may activate" — not supported; modeled as controller-only activation.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Warmonger");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let monger = reg.interner_mut().intern("Monger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(monger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature deals 1 damage to each creature without flying and each player.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_1_to_all,
            }),
    )
}

fn deal_1_to_all(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "without flying" — using all creatures.
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let all_players = script::all_players(state);
    let mut effects: Vec<Effect> = creature_ids
        .into_iter()
        .map(|id| Effect::DealDamage {
            target: DamageTarget::Object(id),
            amount: 1,
            source: ctx.source,
        })
        .collect();
    for p in all_players {
        effects.push(Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 1,
            source: ctx.source,
        });
    }
    vec![Effect::Sequence(effects)]
}
