//! Ever After — `{4}{B}{B}` sorcery. "Return up to two target creature
//! cards from your graveyard to the battlefield. Each of those creatures
//! is a black Zombie in addition to its other colors and types. Put Ever
//! After on the bottom of its owner's library."
//!
//! GAP: returning a specific number (up to 2) of graveyard targets and
//! applying a color/type overlay ('is a black Zombie in addition to') are
//! not expressible with the catalog. Resolving each target with
//! ReturnFromGraveyardToBattlefield and ignoring the color/type rider is
//! the closest expressible approximation; the self-library placement is
//! also a GAP (no Effect for 'put the spell itself on the bottom').

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creature cards from your graveyard to the battlefield. Each of those creatures is a black Zombie in addition to its other colors and types. Put Ever After on the bottom of its owner's library.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                        count: TargetCount::UpTo(2),
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
    // GAP: 'is a black Zombie in addition to' color/type overlay not expressible
    // GAP: 'put Ever After on the bottom of its owner's library' self-placement not expressible
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
        } else {
            None
        }
    }).collect()
}
