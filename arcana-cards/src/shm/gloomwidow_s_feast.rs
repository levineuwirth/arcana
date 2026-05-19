//! Gloomwidow's Feast — `{3}{G}` instant, "Destroy target creature with flying. If that
//! creature was blue or black, create a 1/2 green Spider creature token with reach."
//!
//! GAP: Conditional token creation based on the destroyed creature's colors.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gloomwidow's Feast");
    let _spider = reg.interner_mut().intern("Spider");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature with flying. If that creature was blue or black, create a 1/2 green Spider creature token with reach.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let spider = reg.interner().lookup("Spider").expect("Spider interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    let token = TokenDefinition {
        name: spider,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        abilities: vec![],
    };
    // GAP: conditional token creation based on whether the destroyed creature was blue or black
    vec![Effect::DestroyPermanent { target: *id }]
    // token dropped; conditional on destroyed creature's color not expressible
    // if it were unconditional: vec![Effect::DestroyPermanent { target: *id }, Effect::CreateToken { controller: entry.controller, token }]
}
