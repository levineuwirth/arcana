//! Gangrenous Zombies — `{1}{B}{B}` 2/2 Zombie.
//! `{T}, Sacrifice this creature: This creature deals 1 damage to each creature and each player.
//! If you control a snow Swamp, this creature deals 2 damage to each creature and each player instead.`
//! GAP: "if you control a snow Swamp" — no ObjectFilter for snow supertype on a land subtype.
//! GAP: "damage to each creature and each player" — no Effect::DealDamageToAll combining objects+players.
//! We emit the base 1 damage to each player via ForEach approximation.

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
    let name = reg.interner_mut().intern("Gangrenous Zombies");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice this creature: This creature deals 1 damage to each creature and each player. If you control a snow Swamp, deals 2 instead.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_damage_to_all,
            }),
    )
}

fn deal_damage_to_all(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you control a snow Swamp" — no snow-land filter; using base 1 damage.
    // GAP: "damage to each creature" — ForEach over creature ids approximates this.
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let mut effects: Vec<Effect> = creature_ids
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 1,
        })
        .collect();
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 1,
        });
    }
    effects
}
