//! Eye to Eye — `{2}{B}` instant, "You and target creature's
//! controller have a staring contest. If you win, destroy that
//! creature." The "staring contest" is a 50/50 randomized outcome
//! with no enduring downside on a loss, modeled with `Effect::FlipCoin`:
//! on a win, destroy the targeted creature; on a loss, nothing happens.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eye to Eye");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You and target creature's controller have a staring contest. \
                   If you win, destroy that creature."
                .into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::FlipCoin {
        player: entry.controller,
        win: Box::new(Effect::DestroyPermanent { target: *id }),
        lose: None,
    }]
}
