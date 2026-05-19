//! Splash Portal — `{U}` sorcery, "Exile target creature you control, then
//! return it to the battlefield under its owner's control. If that creature
//! is a Bird, Frog, Otter, or Rat, draw a card."
//! GAP: conditional draw based on creature's subtype (Bird/Frog/Otter/Rat
//! check at resolution not in script helpers).
//! GAP: exile then immediately return (no combined Effect variant; would
//! need ExilePermanent + ReturnFromGraveyardToBattlefield but the creature
//! goes to exile not graveyard).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Splash Portal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature you control, then return it to the battlefield under its owner's control. If that creature is a Bird, Frog, Otter, or Rat, draw a card.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: exile then return from exile to battlefield (no combined Effect variant)
    // GAP: conditional draw if creature is Bird/Frog/Otter/Rat
    vec![Effect::ExilePermanent { target: *id }]
}
