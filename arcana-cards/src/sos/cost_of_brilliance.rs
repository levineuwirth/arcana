//! Cost of Brilliance — `{2}{B}` sorcery. "Target player draws two
//! cards and loses 2 life. Put a +1/+1 counter on up to one target
//! creature."
//!
//! GAP: the second "up to one target creature" is a separate optional
//! target the engine model can't couple here; only the first target
//! (the player) is taken, so the +1/+1 counter clause is gapped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cost of Brilliance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player draws two cards and loses 2 life. Put a +1/+1 counter on up to one target creature.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: second "up to one target creature" +1/+1 counter clause not coupled.
    vec![
        Effect::DrawCards { player: *p, count: 2 },
        Effect::LoseLife { player: *p, amount: 2 },
    ]
}
