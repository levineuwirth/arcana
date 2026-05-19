//! Join Shields — `{3}{G}{W}` instant. "Untap all creatures you control. They
//! gain hexproof and indestructible until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Join Shields");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Untap all creatures you control. They gain hexproof and indestructible until end of turn.".into(),
                target_requirements: vec![],
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
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::Untap { target: id });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Hexproof,
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
