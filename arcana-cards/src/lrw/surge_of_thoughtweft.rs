//! Surge of Thoughtweft — `{1}{W}` Kindred Instant — Kithkin.
//! "Creatures you control get +1/+1 until end of turn. If you
//! control a Kithkin, draw a card."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surge of Thoughtweft");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Creatures you control get +1/+1 until end of turn. If you control a Kithkin, draw a card.".into(),
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
    let mine = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut out: Vec<Effect> = mine
        .into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect();
    let kithkin =
        script::count_matching(state, &script::subtype_filter(reg, "Kithkin"), entry.controller);
    if kithkin > 0 {
        out.push(Effect::DrawCards { player: entry.controller, count: 1 });
    }
    out
}
