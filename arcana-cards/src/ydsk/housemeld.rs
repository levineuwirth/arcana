//! Housemeld — `{2}{U}{U}` sorcery. "Exile target creature. The
//! exiled card perpetually becomes an enchantment. At the beginning
//! of your next end step, put it onto the battlefield under your
//! control."
//!
//! We exile the targeted creature and schedule its return at the
//! next end step. The "perpetually becomes an enchantment / loses
//! all other card types" transformation is not expressible.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Housemeld");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target creature. The exiled card perpetually becomes an enchantment. At the beginning of your next end step, put it onto the battlefield under your control.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "perpetually becomes an enchantment, loses all other card
    // types" and return-under-your-control are not expressible; the
    // delayed action returns it to its owner's hand as the closest
    // available primitive is not a battlefield return — so only the
    // exile + a next-end-step return-to-hand are modeled.
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        },
    ]
}
