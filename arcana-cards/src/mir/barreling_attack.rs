//! Barreling Attack — `{2}{R}{R}` instant. "Target creature gains
//! trample until end of turn. When that creature becomes blocked this
//! turn, it gets +1/+1 until end of turn for each creature blocking
//! it."
//!
//! The trample grant is expressible directly. The delayed
//! "becomes-blocked" rider with a per-blocker dynamic pump is not
//! expressible from a spell resolver (no delayed becomes-blocked
//! trigger primitive in the catalog), so it is GAP-ed.

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
    let name = reg.interner_mut().intern("Barreling Attack");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gains trample until end of turn. When that creature becomes blocked this turn, it gets +1/+1 until end of turn for each creature blocking it.".into(),
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
    // GAP: delayed "when that creature becomes blocked this turn, it gets
    // +1/+1 for each creature blocking it" — no delayed becomes-blocked
    // trigger primitive with a per-blocker dynamic count is expressible
    // from a spell resolver.
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Trample,
        duration: Duration::EndOfTurn,
    }]
}
