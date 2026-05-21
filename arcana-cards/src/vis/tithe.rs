//! Tithe — `{W}` instant. "Search your library for a Plains card. If
//! target opponent controls more lands than you, you may search your
//! library for an additional Plains card. Reveal those cards, put them
//! into your hand, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tithe");
    let _plains = reg.interner_mut().intern("Plains");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for a Plains card. If target opponent controls more lands than you, you may search your library for an additional Plains card. Reveal those cards, put them into your hand, then shuffle.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // Best-effort: the conditional second search depends on a land-count
    // comparison the catalog cannot perform inside a Conditional, so only
    // the unconditional first Plains search is emitted.
    // GAP: "if opponent controls more lands than you" land-count
    // comparison; second Plains search omitted.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: script::subtype_filter(reg, "Plains"),
        reveal: true,
    }]
}
