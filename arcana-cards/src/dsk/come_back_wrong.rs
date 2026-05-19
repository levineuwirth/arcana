//! Come Back Wrong — `{2}{B}` sorcery, "Destroy target creature. If a
//! creature card is put into a graveyard this way, return it to the
//! battlefield under your control. Sacrifice it at the beginning of your
//! next end step."
//!
//! # GAP
//! * GAP: triggered "if creature card put into graveyard this way" zone-change trigger
//! * GAP: return to battlefield under controller's control from that trigger
//! * GAP: delayed "sacrifice at beginning of your next end step" trigger

use arcana_core::effects::Effect;
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
    vec![
        Effect::DestroyPermanent { target: *id },
        // GAP: triggered "if creature card put into graveyard this way" zone-change trigger
        // GAP: return to battlefield under controller's control from that trigger
        // GAP: delayed "sacrifice at beginning of your next end step" trigger
    ]
}
