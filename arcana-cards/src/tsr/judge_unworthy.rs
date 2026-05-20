//! Judge Unworthy — `{1}{W}` instant. "Choose target attacking or
//! blocking creature. Scry 3, then reveal the top card of your
//! library. Judge Unworthy deals damage equal to that card's mana
//! value to that creature."
//!
//! GAP note: the attacking/blocking restriction on the target and the
//! "damage equal to revealed card's mana value" are not expressible
//! with the available target filters / script helpers; modelled as a
//! plain creature target plus Scry 3. The variable damage is GAPped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target attacking or blocking creature. Scry 3, then reveal the top card of your library. Judge Unworthy deals damage equal to that card's mana value to that creature.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(_id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: cannot read the revealed top card's mana value to set the
    // damage amount; emit only the Scry portion.
    vec![Effect::Scry {
        player: entry.controller,
        count: 3,
    }]
}
