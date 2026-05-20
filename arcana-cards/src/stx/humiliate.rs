//! Humiliate — `{W}{B}` sorcery, "Target opponent reveals their hand.
//! You choose a nonland card from it. That player discards that card.
//! Put a +1/+1 counter on a creature you control." Targeted-discard of
//! a caster-chosen card is approximated as a generic discard; the
//! +1/+1 counter targets a creature.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Humiliate");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target opponent reveals their hand. You choose a nonland card from it. That player discards that card. Put a +1/+1 counter on a creature you control.".into(),
            target_requirements: vec![
                TargetRequirement::target_player(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.first() {
        // Caster-chosen nonland card from a revealed hand is approximated.
        out.push(Effect::Discard {
            player: *p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.get(1) {
        out.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    out
}
