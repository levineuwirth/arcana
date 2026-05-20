//! Come Back Wrong — `{2}{B}` sorcery. "Destroy target creature. If a creature
//! card is put into a graveyard this way, return it to the battlefield under
//! your control. Sacrifice it at the beginning of your next end step."
//!
//! GAP: the reanimation-after-destroy is conditional on the card going to the
//! graveyard (i.e. not being indestructible). No Effect::Conditional condition
//! for "card was put into graveyard by this effect" is available. Emitting the
//! destroy + reanimate + sacrifice as unconditional best-effort (the verify
//! pipeline will flag this).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Come Back Wrong");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. If a creature card is put into a graveyard this way, return it to the battlefield under your control. Sacrifice it at the beginning of your next end step.".into(),
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
    // GAP: conditional on card actually dying; emitting unconditionally as best-effort
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
