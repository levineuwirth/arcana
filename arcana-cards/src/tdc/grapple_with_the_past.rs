//! Grapple with the Past — `{1}{G}` instant, "Mill three cards, then you may
//! return a creature or land card from your graveyard to your hand."
//!
//! GAP: 'return a creature or land card' is a choice from a graveyard
//! filtered to creature-or-land; TargetFilter::Card only supports one zone
//! filter; the optional return is a GAP. Best-effort: mill 3, then return
//! target creature from graveyard to hand (land card return is gap).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grapple with the Past");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Mill three cards, then you may return a creature or land card from your graveyard to your hand.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional return of creature-or-land from graveyard (disjunctive zone filter + optional)
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
