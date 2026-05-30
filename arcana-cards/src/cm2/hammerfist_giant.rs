//! Hammerfist Giant — `{4}{R}{R}` 5/4 Giant Warrior.
//! `{T}: Hammerfist Giant deals 4 damage to each creature without flying
//! and each player.`
//! GAP: ObjectFilter has no `.without_keywords(Flying)` refinement; the
//! creature damage currently hits ALL creatures (including flyers).

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
    let name = reg.interner_mut().intern("Hammerfist Giant");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Hammerfist Giant deals 4 damage to each creature without flying and each player.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_4_to_nonflyers_and_players,
            }),
    )
}

fn deal_4_to_nonflyers_and_players(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no ObjectFilter refinement to exclude flying creatures;
    // this hits all creatures (including flyers) instead of only non-flyers.
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);

    let mut effects: Vec<Effect> = creature_ids
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 4,
        })
        .collect();

    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 4,
        });
    }

    effects
}
