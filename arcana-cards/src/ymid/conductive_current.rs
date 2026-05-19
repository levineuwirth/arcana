//! Conductive Current — `{R}{R}{R}` sorcery.
//! "Conductive Current deals 3 damage to each creature. Choose an instant or sorcery
//! card in your hand. It perpetually gains 'If this spell would deal noncombat damage
//! to a permanent or player, it deals that much damage plus 2 instead.'"
//!
//! GAP: "perpetually gains" a rules-text ability on a card in hand — no PerpetualEffect
//! or GrantAbilityToCardInHand variant in catalog.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::objects::NULL_OBJECT_ID;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conductive Current");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Conductive Current deals 3 damage to each creature. Choose an instant or sorcery card in your hand. It perpetually gains 'If this spell would deal noncombat damage to a permanent or player, it deals that much damage plus 2 instead.'".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    use arcana_core::script;
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut effects: Vec<Effect> = ids.into_iter().map(|id| Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(id),
        amount: 3,
    }).collect();
    // GAP: "choose instant/sorcery in hand, it perpetually gains ..." — no
    // PerpetualEffect / GrantTextAbilityToHandCard variant in catalog.
    effects
}
