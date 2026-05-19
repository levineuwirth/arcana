//! Careful Consideration — `{2}{U}{U}` instant, "Target player draws four
//! cards, then discards three cards. If it's your main phase, instead that
//! player draws four cards, then discards two cards."
//! GAP: main-phase conditional on discard count not expressible; best effort
//! uses the non-main-phase clause (draw 4, discard 3).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Careful Consideration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player draws four cards, then discards three cards. If it's your main phase, instead that player draws four cards, then discards two cards.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let player = match target {
        arcana_core::targets::TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    // GAP: main-phase conditional on discard count not expressible; using draw 4 discard 3
    vec![
        Effect::DrawCards { player, count: 4 },
        Effect::Discard { player, count: 3, choice: DiscardChoice::ControllerChooses },
    ]
}
