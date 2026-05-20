//! Make Your Mark — `{R/W}` instant. "Target creature gets +1/+0
//! until end of turn. When that creature dies this turn, create a 3/2
//! red and white Spirit creature token." The delayed dies-trigger
//! token is not expressible (DelayedAction has no token-create
//! action); we emit the +1/+0 pump.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Make Your Mark");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +1/+0 until end of turn. When that creature dies this turn, create a 3/2 red and white Spirit creature token.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "when that creature dies this turn, create a token" — a
    // delayed dies-triggered token; DelayedAction has no token-create
    // action variant. Pump is emitted.
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
