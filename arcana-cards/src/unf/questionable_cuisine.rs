//! Questionable Cuisine — `{3}{B}` sorcery. "Return up to two target creature cards from your
//! graveyard to your hand. Create a Food token for each trash can you can see from your seat."
//! The Food token creation clause depends on a real-world observable condition (trash cans
//! visible from seat) and is not modeled. The graveyard return portion is fully expressible.
//! GAP: create Food tokens based on real-world observable condition — not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Questionable Cuisine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creature cards from your graveyard to your hand. Create a Food token for each trash can you can see from your seat.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
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
    let mut effects: Vec<Effect> = entry.targets.targets.iter().filter_map(|t| {
        use arcana_core::targets::TargetChoice;
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToHand { target: *id })
        } else {
            None
        }
    }).collect();
    // GAP: create Food tokens based on real-world observable condition (trash cans visible
    // from seat) — not expressible
    effects
}
