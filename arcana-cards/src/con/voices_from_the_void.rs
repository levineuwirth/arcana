//! Voices from the Void — `{4}{B}` sorcery. "Domain — Target player
//! discards a card for each basic land type among lands you control."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voices from the Void");
    for t in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        reg.interner_mut().intern(t);
    }
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Target player discards a card for each basic land type among lands you control.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // Domain: count distinct basic land types among lands you control.
    let mut domain = 0u32;
    for t in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let n = script::count_matching(
            state,
            &script::subtype_filter(reg, t)
                .controlled_by(ControllerConstraint::You),
            entry.controller,
        );
        if n > 0 {
            domain += 1;
        }
    }
    vec![Effect::Discard {
        player: *p,
        count: domain,
        choice: DiscardChoice::ControllerChooses,
    }]
}
