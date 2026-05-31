//! Strength in Numbers — `{1}{G}` instant. "Until end of turn, target
//! creature gains trample and gets +X/+X, where X is the number of
//! attacking creatures."
//!
//! The trample grant is fully expressible. The +X/+X rider is dynamic,
//! scaling with the number of attacking creatures — a quantity that has
//! no `script::*` helper (there is no attacking-creature count), so the
//! pump is GAP-ed rather than emitted with a wrong literal.

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
    let name = reg.interner_mut().intern("Strength in Numbers");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, target creature gains trample and gets +X/+X, where X is the number of attacking creatures.".into(),
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
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: +X/+X where X is the number of attacking creatures — there is
    // no script:: helper that counts attacking creatures, so the dynamic
    // pump cannot be computed. Emitting only the trample grant.
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Trample,
        duration: Duration::EndOfTurn,
    }]
}
