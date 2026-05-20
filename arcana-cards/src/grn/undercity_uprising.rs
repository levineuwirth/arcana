//! Undercity Uprising — `{2}{B}{G}` sorcery. Colors: B, G.
//! "Creatures you control gain deathtouch until end of turn. Then target creature
//! you control fights target creature you don't control."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undercity Uprising");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control gain deathtouch until end of turn. Then target creature you control fights target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::You)),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
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
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let grant_deathtouch = Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        }),
    };

    let target_a = entry.targets.targets.first();
    let target_b = entry.targets.targets.get(1);
    match (target_a, target_b) {
        (Some(TargetChoice::Object(id_a)), Some(TargetChoice::Object(id_b))) => {
            vec![
                grant_deathtouch,
                Effect::Fight { a: *id_a, b: *id_b },
            ]
        }
        _ => vec![grant_deathtouch],
    }
}
