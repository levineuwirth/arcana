//! Squallmonger — `{3}{G}` 3/3 Creature — Monger.
//! `{2}: This creature deals 1 damage to each creature with flying and each player. Any player may activate this ability.`
//! GAP: "any player may activate" — ActivationZone doesn't support opponent-activatable abilities.
//! GAP: "deals 1 damage to each creature with flying" — no ObjectFilter for "has flying keyword".

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Squallmonger");
    let monger = reg.interner_mut().intern("Monger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(monger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature deals 1 damage to each creature with flying and each player.".into(),
                cost: ActivationCost { mana_cost: ManaCost::parse("{2}").unwrap(), ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_to_flyers_and_players,
            }),
    )
}

fn deal_to_flyers_and_players(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "creatures with flying" — no ObjectFilter method for "has Flying keyword"
    // Approximating as all creatures + all players
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let all_players = script::all_players(state);
    let mut effects: Vec<Effect> = creature_ids.into_iter().map(|id| Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(id),
        amount: 1,
    }).collect();
    for p in all_players {
        effects.push(Effect::DealDamage { source: ctx.source, target: DamageTarget::Player(p), amount: 1 });
    }
    effects
}
