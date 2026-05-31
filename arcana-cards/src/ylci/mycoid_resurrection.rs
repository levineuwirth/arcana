//! Mycoid Resurrection — `{4}{B}{G}` sorcery. "Fathomless descent —
//! Each creature card in your graveyard perpetually gets +X/+X, where X
//! is the number of permanent cards in your graveyard. Then return a
//! creature card from your graveyard to the battlefield."
//!
//! The reanimation half is expressible: target a creature card in a
//! graveyard and return it with `Effect::ReturnFromGraveyardToBattlefield`.
//! The "perpetually +X/+X to each creature card in your graveyard" half is
//! GAP'd: there is no perpetual / graveyard-wide buff primitive.

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
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mycoid Resurrection");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Fathomless descent — Each creature card in your graveyard perpetually \
                   gets +X/+X, where X is the number of permanent cards in your graveyard. \
                   Then return a creature card from your graveyard to the battlefield."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "each creature card in your graveyard perpetually gets +X/+X"
    // clause is not expressible — there is no perpetual graveyard-wide buff
    // primitive. Only the reanimation half is emitted.
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
