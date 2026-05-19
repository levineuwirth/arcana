//! Rain of Daggers — `{4}{B}{B}` sorcery.
//! "Destroy all creatures target opponent controls. You lose 2 life for each
//! creature destroyed this way."
//! GAP: 'you lose 2 life per creature destroyed' (count-dependent life loss
//! based on opponent-controlled creatures — script::ids_matching can count
//! them, but the target is 'target opponent', not entry.controller, and
//! per-creature life loss after ForEach is not directly expressible).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rain of Daggers");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures target opponent controls. You lose 2 life for each creature destroyed this way.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let TargetChoice::Player(opponent) = target else { return Vec::new(); };
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, *opponent);
    let count = ids.len() as u32;
    // GAP: life loss per creature destroyed (count-dependent life loss after ForEach)
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
        Effect::LoseLife { player: entry.controller, amount: count * 2 },
    ]
}
