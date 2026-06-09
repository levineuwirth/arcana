//! Barge In — `{R}` instant. "Target attacking creature gets +2/+2
//! until end of turn. Each attacking non-Human creature gains trample
//! until end of turn."
//! "non-Human" wired via ObjectFilter::without_subtype_sym.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barge In");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target attacking creature gets +2/+2 until end of turn. Each attacking non-Human creature gains trample until end of turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                count: TargetCount::Exactly(1),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    // "non-Human": exclude the Human subtype via without_subtype_sym.
    let mut attacking_non_human = ObjectFilter::creature().attacking_only();
    if let Some(human) = reg.interner().lookup("Human") {
        attacking_non_human = attacking_non_human.without_subtype_sym(human);
    }
    let attackers = script::ids_matching(state, &attacking_non_human, entry.controller);
    for aid in attackers {
        effects.push(Effect::GrantKeyword {
            target: aid,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
