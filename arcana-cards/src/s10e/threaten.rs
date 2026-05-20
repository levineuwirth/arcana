//! Threaten — `{2}{R}` sorcery. "Untap target creature and gain
//! control of it until end of turn. That creature gains haste until
//! end of turn."
//!
//! GAP: temporary "gain control until end of turn" has no catalog
//! Effect; the untap and the haste grant are emitted.

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
    let name = reg.interner_mut().intern("Threaten");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Untap target creature and gain control of it until end of turn. That creature gains haste until end of turn.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: temporary control-change ("gain control until end of turn") not expressible.
    vec![
        Effect::Untap { target: *id },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Haste, duration: Duration::EndOfTurn },
    ]
}
