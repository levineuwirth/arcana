//! Fell the Pheasant — `{1}{G}` instant. Deals 5 damage to target creature
//! with flying. Create a Food token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fell the Pheasant");
    let _ = reg.interner_mut().intern("Food");
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
                text: "Fell the Pheasant deals 5 damage to target creature with flying. Create a Food token.".into(),
                target_requirements: vec![TargetRequirement {
                    // "target creature with flying" — enforced via keyword filter.
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let food = reg.interner().lookup("Food")
        .expect("Food interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 5,
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
