//! Return Triumphant — `{1}{W}` sorcery. "Return target creature card
//! with mana value 3 or less from your graveyard to the battlefield.
//! Create a Young Hero Role token attached to it."
//!
//! GAP note: the Young Hero Role token (an Aura-like attached
//! enchantment token granting a triggered ability) is not expressible
//! with the token/aura API available here. Only the reanimation is
//! emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Return Triumphant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature card with mana value 3 or less from your graveyard to the battlefield. Create a Young Hero Role token attached to it.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_cmc(3),
                },
                count: arcana_core::targets::TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: Young Hero Role token (attached Aura-style enchantment
    // token with a triggered ability) is not expressible.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
