//! Martyr of Ashes — `{R}` 1/1 Human Shaman.
//! `{2}, Reveal X red cards from your hand, Sacrifice this creature:` This
//! creature deals X damage to each creature without flying.
//! GAP: "reveal X red cards from your hand" as activation cost not
//! expressible, so `x_value` is never set and the damage stays at 0.

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
    let name = reg.interner_mut().intern("Martyr of Ashes");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Reveal X red cards from your hand, Sacrifice this creature: This creature deals X damage to each creature without flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: x_damage_grounders,
            }),
    )
}

fn x_damage_grounders(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal X red cards from hand" as cost not expressible, so
    // ctx.x_value is never populated and X stays 0.
    let amount = ctx.x_value.unwrap_or(0);
    if amount == 0 {
        return Vec::new();
    }
    let filter = ObjectFilter::creature().without_keyword(KeywordAbility::Flying);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount,
        })
        .collect()
}
