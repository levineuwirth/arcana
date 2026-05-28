//! Krark-Clan Shaman — `{R}` 1/1 red Goblin Shaman.
//! `Sacrifice an artifact: This creature deals 1 damage to each
//! creature without flying.`
//!
//! GAP: ActivationCost.sacrifice sacrifices self. "Sacrifice an
//! artifact" (not self) is not expressible.

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
    let name = reg.interner_mut().intern("Krark-Clan Shaman");
    let goblin = reg.interner_mut().intern("Goblin");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
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
                text: "Sacrifice an artifact: This creature deals 1 damage to each creature without flying.".into(),
                // GAP: "sacrifice an artifact" (not self).
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_one_to_nonflyers,
            }),
    )
}

fn deal_one_to_nonflyers(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Filter creatures without flying — ObjectFilter doesn't have a
    // "without_keyword" filter, using all creatures as best effort.
    // GAP: cannot filter "without flying" specifically.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            target: DamageTarget::Object(arcana_core::objects::NULL_OBJECT_ID),
            amount: 1,
            source: ctx.source,
        }),
    }]
}
