//! Truce — `{2}{W}` instant. Each player may draw up to two cards. For
//! each card less than two a player draws this way, that player gains
//! 2 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Truce");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player may draw up to two cards. For each card less than two a player draws this way, that player gains 2 life.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Best-effort: each player draws 2; cannot model "up to" / per-player
    // shortfall life gain. GAP: optional draw choice + tied life-gain rider.
    vec![Effect::Sequence(
        script::all_players(state)
            .into_iter()
            .map(|p| Effect::DrawCards { player: p, count: 2 })
            .collect(),
    )]
}
