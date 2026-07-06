//! Haunt the Network — `{3}{U}{B}` sorcery. "Choose target opponent.
//! Create two 1/1 colorless Thopter artifact creature tokens with
//! flying. Then the chosen player loses X life and you gain X life,
//! where X is the number of artifacts you control."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haunt the Network");
    let _thopter = reg.interner_mut().intern("Thopter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target opponent. Create two 1/1 colorless Thopter artifact creature tokens with flying. Then the chosen player loses X life and you gain X life, where X is the number of artifacts you control.".into(),
            target_requirements: vec![TargetRequirement::target_opponent()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let thopter = reg
        .interner()
        .lookup("Thopter")
        .expect("Thopter interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);
    let token = TokenDefinition {
        name: thopter,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    let x = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let chosen = match entry.targets.targets.first() {
        Some(TargetChoice::Player(p)) => *p,
        _ => return Vec::new(),
    };
    vec![
        Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: entry.controller,
            token,
        },
        Effect::LoseLife {
            player: chosen,
            amount: x,
        },
        Effect::GainLife {
            player: entry.controller,
            amount: x,
        },
    ]
}
