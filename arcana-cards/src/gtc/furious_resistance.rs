//! Furious Resistance — `{R}` instant. "Target blocking creature gets
//! +3/+0 and gains first strike until end of turn."
//!
//! GAP: TargetBlockingCreature (targeting specifically a creature that
//! is currently blocking) — TargetFilter::Creature does not constrain
//! to blocking status; no TargetFilter variant for blocking creatures.
//! Using TargetFilter::Creature as approximation.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Furious Resistance");
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
                text: "Target blocking creature gets +3/+0 and gains first strike until end of turn.".into(),
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
    // GAP: TargetBlockingCreature (no TargetFilter variant restricts to blocking creatures)
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike],
    }]
}
