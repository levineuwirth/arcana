//! Roiling Waters — `{5}{U}{U}` sorcery. "Return up to two target
//! creatures your opponents control to their owners' hands. Target
//! player draws two cards."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roiling Waters");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creatures your opponents control to their owners' hands. Target player draws two cards.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::UpTo(2),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Player,
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();
    // First N (up to 2) targets are creatures, last is the player.
    let len = entry.targets.targets.len();
    if len == 0 {
        return Vec::new();
    }
    for t in entry.targets.targets.iter().take(len.saturating_sub(1)) {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnToHand { target: *id });
        }
    }
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.last() {
        effects.push(Effect::DrawCards { player: *p, count: 2 });
    }
    effects
}
