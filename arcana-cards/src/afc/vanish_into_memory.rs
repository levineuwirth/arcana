//! Vanish into Memory — `{2}{W}{U}` instant. Colors: W, U.
//! "Exile target creature. You draw cards equal to that creature's power.
//! At the beginning of your next upkeep, return that card to the battlefield
//! under its owner's control. If you do, discard cards equal to that
//! creature's toughness."
//!
//! The draw is dynamic (equal to creature's power). We use script::power_of.
//! The return at next upkeep + conditional discard: DelayedAction supports
//! NextEndStep but NOT NextUpkeep. GAP for the delayed return+discard.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vanish into Memory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature. You draw cards equal to that creature's power. At the beginning of your next upkeep, return that card to the battlefield under its owner's control. If you do, discard cards equal to that creature's toughness.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let power = script::power_of(state, *id).max(0) as u32;
    // GAP: "at beginning of your next upkeep, return + discard toughness" —
    // DelayedWhen::NextUpkeep not in catalog (only NextEndStep / ThisDies)
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DrawCards { player: entry.controller, count: power },
    ]
}
