//! Strength in Numbers — `{1}{G}` instant. "Until end of turn, target
//! creature gains trample and gets +X/+X, where X is the number of
//! attacking creatures."
//!
//! Both halves are expressible: trample via `GrantKeyword`, and X via
//! counting `ObjectFilter::creature().attacking_only()`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    let x = script::count_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        entry.controller,
    ) as i32;
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
        Effect::Pump {
            target: *id,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
