//! Rally the Righteous — `{1}{R}{W}` instant, "Radiance — Untap target creature and each other
//! creature that shares a color with it. Those creatures get +2/+0 until end of turn."
//!
//! GAP: Radiance mechanic (affect target and each other permanent sharing a color with it)
//! requires querying color-sharing permanents; no catalog Effect or script helper covers this.
//! Only the single-target Untap + Pump is implemented for the primary target.

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
    let name = reg.interner_mut().intern("Rally the Righteous");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Radiance — Untap target creature and each other creature that shares a color with it. Those creatures get +2/+0 until end of turn.".into(),
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
    // GAP: Radiance — untap/pump each other creature sharing a color with the target
    vec![
        Effect::Untap { target: *id },
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
