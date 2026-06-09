//! Glyph of Destruction — `{R}` instant. "Target blocking Wall you
//! control gets +10/+0 until end of combat. Prevent all damage that
//! would be dealt to it this turn. Destroy it at the beginning of the
//! next end step." Until-end-of-combat duration isn't in Duration
//! (only EndOfTurn), so the pump uses EndOfTurn. We emit the
//! +10/+0 EOT, the prevention, and the delayed self-destroy.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glyph of Destruction");
    let wall_filter = script::subtype_filter(reg, "Wall")
        .controlled_by(ControllerConstraint::You)
        .blocking_only();
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target blocking Wall you control gets +10/+0 until end of combat. Prevent all damage that would be dealt to it this turn. Destroy it at the beginning of the next end step.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(wall_filter),
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
    // GAP: 'until end of combat' duration not in Duration — we use
    // EndOfTurn for the pump and emit prevention + delayed destroy.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Pump { target: *id, power: 10, toughness: 0, duration: Duration::EndOfTurn, keywords: vec![] },
        Effect::PreventDamage { target: DamageTarget::Object(*id), amount: None, duration: ReplacementDuration::EndOfTurn },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
