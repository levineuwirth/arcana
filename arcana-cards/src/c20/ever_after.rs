//! Ever After — `{4}{B}{B}` sorcery. "Return up to two target creature
//! cards from your graveyard to the battlefield. Each of those creatures
//! is a black Zombie in addition to its other colors and types. Put Ever
//! After on the bottom of its owner's library."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ever After");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to two target creature cards from your graveyard \
                   to the battlefield. Each of those creatures is a black \
                   Zombie in addition to its other colors and types. Put Ever \
                   After on the bottom of its owner's library."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::UpTo(2),
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
    // GAP: cannot grant "is a black Zombie in addition" to the returned
    // cards, nor put the spell itself on the bottom of its owner's library.
    let mut effects = Vec::new();
    for t in entry.targets.targets.iter() {
        if let arcana_core::targets::TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    effects
}
