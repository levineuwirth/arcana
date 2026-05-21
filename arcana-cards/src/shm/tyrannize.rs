//! Tyrannize — `{3}{B/R}{B/R}` sorcery. "Target player discards their hand
//! unless they pay 7 life." The pay-life-or-discard choice has no catalog
//! Effect, so it is modeled as the target player discarding their hand.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyrannize");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player discards their hand unless they pay 7 life.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "unless they pay 7 life" optional cost has no catalog Effect; modeled
    // as the target player discarding their whole hand.
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Discard {
        player: *p,
        count: script::hand_size(state, *p),
        choice: DiscardChoice::ControllerChooses,
    }]
}
