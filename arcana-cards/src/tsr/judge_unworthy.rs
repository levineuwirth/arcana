//! Judge Unworthy — `{1}{W}` instant. "Choose target attacking or
//! blocking creature. Scry 3, then reveal the top card of your library.
//! Judge Unworthy deals damage equal to that card's mana value to that
//! creature." The combat-state restriction is enforced via
//! `ObjectFilter::creature().attacking_or_blocking_only()`. Mana
//! value of revealed top card isn't a script helper. Emit the
//! Scry 3 + GAP the dynamic damage.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Judge Unworthy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target attacking or blocking creature. Scry 3, then reveal the top card of your library. Judge Unworthy deals damage equal to that card's mana value to that creature.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().attacking_or_blocking_only(),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: mana-value of the revealed top library card driving the
    // damage amount.
    vec![Effect::Scry { player: entry.controller, count: 3 }]
}
