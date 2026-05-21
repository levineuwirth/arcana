//! Trade Secrets — `{1}{U}{U}` sorcery. "Target opponent draws two
//! cards, then you draw up to four cards. That opponent may repeat
//! this process as many times as they choose."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trade Secrets");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target opponent draws two cards, then you draw up to four cards. That opponent may repeat this process as many times as they choose.".into(),
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
    let TargetChoice::Player(opp) = target else { return Vec::new(); };
    // GAP: "may repeat as many times as they choose" loop primitive
    // not in the catalog; "up to N" Draw also not modeled. Emit the
    // first round at the fixed counts.
    vec![
        Effect::DrawCards { player: *opp, count: 2 },
        Effect::DrawCards { player: entry.controller, count: 4 },
    ]
}
