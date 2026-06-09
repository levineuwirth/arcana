//! Arachnogenesis — `{2}{G}` instant. "Create X 1/2 green Spider
//! creature tokens with reach, where X is the number of creatures
//! attacking you. Prevent all combat damage that would be dealt this
//! turn by non-Spider creatures."
//!
//! X is counted at resolution via `script::count_matching` with
//! `ObjectFilter::creature().attacking_you_only()`. The combat-damage
//! prevention is expressed with a source-filtered `PreventDamageFrom`
//! whose filter exempts Spiders via `without_subtype_sym`.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arachnogenesis");
    let _spider = reg.interner_mut().intern("Spider");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create X 1/2 green Spider creature tokens with reach, where X is the number of creatures attacking you. Prevent all combat damage that would be dealt this turn by non-Spider creatures.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // X = number of creatures attacking you, counted at resolution.
    let x = script::count_matching(
        state,
        &ObjectFilter::creature().attacking_you_only(),
        entry.controller,
    );
    let spider = reg.interner().lookup("Spider").expect("Spider interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(spider);
    let token = TokenDefinition {
        name: spider,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        abilities: vec![],
    };
    let mut effects: Vec<Effect> = (0..x)
        .map(|_| Effect::CreateToken { controller: entry.controller, token: token.clone() })
        .collect();
    // Prevent all combat damage dealt by non-Spider creatures.
    effects.push(Effect::PreventDamageFrom {
        source_filter: ObjectFilter::creature().without_subtype_sym(spider),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    });
    effects
}
