//! Deadly Embrace — `{3}{B}{B}` sorcery. "Destroy target creature an
//! opponent controls. Then draw a card for each creature that died this
//! turn."
//!
//! The destroy half is expressed; the rider's dynamic draw count comes
//! from `script::creatures_died_this_turn`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deadly Embrace");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature an opponent controls. Then draw a card for each creature that died this turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::DestroyPermanent { target: *id }];
    // "Then draw a card for each creature that died this turn."
    // NOTE: counted at resolution, before the destroy above is applied —
    // the destroyed target itself is not included in the count.
    let died = arcana_core::script::creatures_died_this_turn(state);
    if died > 0 {
        effects.push(Effect::DrawCards {
            player: entry.controller,
            count: died,
        });
    }
    effects
}
