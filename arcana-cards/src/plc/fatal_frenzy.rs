//! Fatal Frenzy — `{2}{R}` instant.
//! "Until end of turn, target creature you control gains trample and gets
//! +X/+0, where X is its power. Sacrifice it at the beginning of the next
//! end step."
//!
//! GAP: "sacrifice at beginning of next end step" delayed trigger not in
//! catalog. Best-effort: pump by current power and grant trample.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fatal Frenzy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, target creature you control gains trample and gets +X/+0, where X is its power. Sacrifice it at the beginning of the next end step.".into(),
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
    let power = script::power_of(state, *id);
    // GAP: "sacrifice at beginning of next end step" delayed trigger not in catalog
    vec![Effect::Pump {
        target: *id,
        power,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}
