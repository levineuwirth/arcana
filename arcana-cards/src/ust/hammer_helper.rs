//! Hammer Helper — `{3}{R}` sorcery. "Gain control of target creature
//! until end of turn. Untap that creature and roll a six-sided die.
//! Until end of turn, it gains haste and gets +X/+0, where X is the
//! result."
//!
//! Partial: the engine has no until-end-of-turn gain-control variant
//! (only permanent ChangeControl), and no six-sided-die roll primitive
//! to source the dynamic +X/+0. Per catalog guidance for the
//! Threaten/Act-of-Treason class, emit the expressible pieces (Untap +
//! grant Haste) and GAP the temporary control and the die-rolled pump
//! rather than emitting a permanent control change or a hardcoded pump.

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
    let name = reg.interner_mut().intern("Hammer Helper");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Gain control of target creature until end of turn. Untap that creature and roll a six-sided die. Until end of turn, it gains haste and gets +X/+0, where X is the result.".into(),
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
    // GAP: no until-end-of-turn gain-control variant (only permanent
    // ChangeControl), and no six-sided-die roll primitive for the
    // dynamic +X/+0. Emitting the expressible Untap + Haste pieces only.
    vec![
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
