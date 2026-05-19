//! On the Job — `{2}{W}{W}` instant.
//! "Creatures you control get +2/+1 until end of turn. Investigate."
//!
//! GAP: Investigate (create a Clue token with activated draw ability) is not
//! directly a catalog Effect. Using CreateToken for the Clue artifact token;
//! the "{2}, Sacrifice: Draw a card" activated ability on the token is not
//! expressible via TokenDefinition.abilities (no ActivatedAbilityDef catalog).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("On the Job");
    let _clue = reg.interner_mut().intern("Clue");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control get +2/+1 until end of turn. Investigate.".into(),
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
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, entry.controller);
    let mut effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 2,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect();
    // Investigate: create a Clue artifact token
    // GAP: Clue token's activated ability "{2}, Sacrifice: Draw a card" not in TokenDefinition
    let clue = reg.interner().lookup("Clue").expect("Clue interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clue);
    let token = TokenDefinition {
        name: clue,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
