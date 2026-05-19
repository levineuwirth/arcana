//! Temporary Insanity — `{3}{R}` instant. "Untap target creature with power
//! less than the number of cards in your graveyard and gain control of it
//! until end of turn. That creature gains haste until end of turn."
//!
//! GAP: 'gain control of target creature until end of turn' effect not
//! in catalog. GAP: targeting constraint 'power less than graveyard size'
//! is a dynamic filter not expressible in TargetRequirement. Approximated
//! with Untap + Haste grant; control effect omitted.

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
    // GAP: gain control until end of turn not expressible
    // GAP: dynamic power < graveyard_size target filter not expressible
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Untap { target: *id },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Haste, duration: Duration::EndOfTurn },
    ]
}
