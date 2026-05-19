//! Soulshriek — `{B}` instant.
//! "Target creature you control gets +X/+0 until end of turn, where X is the
//! number of creature cards in your graveyard. Sacrifice that creature at the
//! beginning of the next end step."
//! GAP: "sacrifice at the beginning of the next end step" delayed trigger not available.

use arcana_core::effects::{Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soulshriek");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +X/+0 until end of turn, where X is the number of creature cards in your graveyard. Sacrifice that creature at the beginning of the next end step.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = script::graveyard_matching(state, &ObjectFilter::creature(), entry.controller, entry.controller);
    // GAP: delayed sacrifice at beginning of next end step not available in Effect catalog
    vec![Effect::Pump {
        target: *id,
        power: x as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
