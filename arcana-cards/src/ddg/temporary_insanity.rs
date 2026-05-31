//! Temporary Insanity — `{3}{R}` instant. "Untap target creature with
//! power less than the number of cards in your graveyard and gain
//! control of it until end of turn. That creature gains haste until
//! end of turn."
//!
//! Implemented partially: we Untap the target and grant it Haste until
//! end of turn. The "gain control until end of turn" piece has no
//! end-of-turn-duration control primitive (only a permanent
//! `ChangeControl`), so it is GAP-ed rather than mis-modeled as a
//! permanent steal. The dynamic target restriction ("power less than
//! the number of cards in your graveyard") is also not expressible as
//! a target filter, so we target any creature.

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
    let name = reg.interner_mut().intern("Temporary Insanity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Untap target creature with power less than the number of cards in your graveyard and gain control of it until end of turn. That creature gains haste until end of turn.".into(),
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
    // GAP: gain control until end of turn — no end-of-turn-duration
    // control primitive (only permanent ChangeControl).
    vec![
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
