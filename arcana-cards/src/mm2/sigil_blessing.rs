//! Sigil Blessing — `{G}{W}` instant.
//! "Until end of turn, target creature you control gets +3/+3 and other
//! creatures you control get +1/+1."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sigil Blessing");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, target creature you control gets +3/+3 and other creatures you control get +1/+1.".into(),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(main_id) = target else { return Vec::new(); };
    let others = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    )
    .into_iter()
    .filter(|id| id != main_id)
    .collect::<Vec<_>>();

    let mut effects = vec![Effect::Pump {
        target: *main_id,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    for id in others {
        effects.push(Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}
