//! Gift of the Viper — `{G}` instant. "Put a +1/+1 counter, a reach
//! counter, and a deathtouch counter on target creature. Untap it."
//! Only +1/+1 counters and keyword grants are in catalog — reach/
//! deathtouch counters are not modeled as counter kinds, so we grant
//! the keywords for end-of-turn and GAP the permanence.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gift of the Viper");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter, a reach counter, and a deathtouch counter on target creature. Untap it.".into(),
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
    // GAP: reach/deathtouch counters as counter kinds (only +1/+1 in
    // catalog) and the permanence of granted keywords (Duration only
    // supports EndOfTurn). We emit +1/+1 counter, EOT keyword grants,
    // and Untap.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Reach, duration: Duration::EndOfTurn },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Deathtouch, duration: Duration::EndOfTurn },
        Effect::Untap { target: *id },
    ]
}
