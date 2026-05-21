//! Timely Reinforcements — `{2}{W}` sorcery. "If you have less life
//! than an opponent, you gain 6 life. If you control fewer creatures
//! than an opponent, create three 1/1 white Soldier creature tokens."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Timely Reinforcements");
    let _soldier = reg.interner_mut().intern("Soldier");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "If you have less life than an opponent, you gain 6 life. If you control fewer creatures than an opponent, create three 1/1 white Soldier creature tokens.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    let my_life = script::life(state, entry.controller);
    let my_creatures = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let opponents = script::opponents(state, entry.controller);

    let any_higher_life = opponents
        .iter()
        .any(|&o| script::life(state, o) > my_life);
    if any_higher_life {
        effects.push(Effect::GainLife { player: entry.controller, amount: 6 });
    }

    let any_more_creatures = opponents.iter().any(|&o| {
        script::count_matching(
            state,
            &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            o,
        ) > my_creatures
    });
    if any_more_creatures {
        let soldier = reg.interner().lookup("Soldier").expect("Soldier interned");
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(soldier);
        let token = TokenDefinition {
            name: soldier,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        };
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
        effects.push(Effect::CreateToken { controller: entry.controller, token });
    }
    effects
}
