//! Descent of the Dragons — `{4}{R}{R}` sorcery. "Destroy any number
//! of target creatures. For each creature destroyed this way, its
//! controller creates a 4/4 red Dragon creature token with flying."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Descent of the Dragons");
    let _dragon = reg.interner_mut().intern("Dragon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy any number of target creatures. For each creature destroyed this way, its controller creates a 4/4 red Dragon creature token with flying.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Any,
                    controller: None,
                }],
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
    let dragon = reg
        .interner()
        .lookup("Dragon")
        .expect("Dragon interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let token_template = TokenDefinition {
        name: dragon,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    let mut effects = Vec::new();
    for choice in &entry.targets.targets {
        if let TargetChoice::Object(id) = choice {
            let id = *id;
            let controller = script::target_controller(state, id, entry.controller);
            effects.push(Effect::DestroyPermanent { target: id });
            effects.push(Effect::CreateToken {
                controller,
                token: token_template.clone(),
            });
        }
    }
    effects
}
