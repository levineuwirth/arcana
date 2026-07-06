//! Curious Herd — `{3}{G}` instant. "Choose target opponent. You
//! create X 3/3 green Beast creature tokens, where X is the number of
//! artifacts that player controls." We approximate the per-target
//! opponent's artifact count via ControllerConstraint::Opponent from
//! the caster's perspective (correct in 1v1, an over-count with
//! multiple opponents).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Curious Herd");
    let _beast = reg.interner_mut().intern("Beast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    // GAP: precise per-target-opponent artifact count — multi-opponent imprecise.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target opponent. You create X 3/3 green Beast creature tokens, where X is the number of artifacts that player controls.".into(),
                target_requirements: vec![TargetRequirement::target_opponent()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("Beast interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let token = TokenDefinition {
        name: beast,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    let mut effects = Vec::with_capacity(n as usize);
    for _ in 0..n {
        effects.push(Effect::CreateToken { controller: entry.controller, token: token.clone() });
    }
    effects
}
