//! Suspend Aggression — `{1}{R}{W}` instant. "Exile target nonland
//! permanent and the top card of your library. For each of those
//! cards, its owner may play it until the end of their next turn."
//!
//! Only the exile of the target nonland permanent is expressible. The
//! "exile the top card of your library" portion and the "its owner may
//! play it until the end of their next turn" play-permission rider have
//! no catalog primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suspend Aggression");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target nonland permanent and the top card of your library. For each of those cards, its owner may play it until the end of their next turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: cannot express "exile the top card of your library" nor the
    // "its owner may play those cards until the end of their next turn"
    // play-permission rider; only the target's exile is emitted.
    vec![Effect::ExilePermanent { target: *id }]
}
