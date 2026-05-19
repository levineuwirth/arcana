//! The Battle of Bywater — `{1}{W}{W}` sorcery. "Destroy all creatures with
//! power 3 or greater. Then create a Food token for each creature you control."
//! GAP: Food token (artifact with sacrifice ability) not in TokenDefinition
//! catalog; creating generic artifact token as best effort.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Battle of Bywater");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures with power 3 or greater. Then create a Food token for each creature you control.".into(),
                target_requirements: vec![],
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
    // GAP: Food token's sacrifice-for-life activated ability not in TokenDefinition
    let big_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().with_min_power(3),
        entry.controller,
    );
    let my_creatures_count = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(arcana_core::targets::ControllerConstraint::You),
        entry.controller,
    );
    let food = reg.interner().lookup("Food")
        .expect("Food interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    let food_token = TokenDefinition {
        name: food,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![Effect::ForEach {
        targets: big_creatures,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    for _ in 0..my_creatures_count {
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: food_token.clone(),
        });
    }
    effects
}
