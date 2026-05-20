//! Hypothesizzle — `{3}{U}{R}` instant.
//! "Draw two cards. Then you may discard a nonland card. When you do,
//! Hypothesizzle deals 4 damage to target creature."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hypothesizzle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw two cards. Then you may discard a nonland card. When you do, Hypothesizzle deals 4 damage to target creature.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ];
    // The discard-conditioned reflexive trigger is approximated as an
    // unconditional discard + damage; the damage is dealt to the
    // target creature.
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 4,
        });
    }
    effects
}
