//! Morgue Burst — `{4}{B}{R}` sorcery, "Return target creature card from your
//! graveyard to your hand. Morgue Burst deals damage to any target equal to
//! the power of the card returned this way."
//!
//! GAP: damage amount = power of the returned card (runtime power lookup on a
//! graveyard card); DealDamage requires a fixed integer amount.

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
    let name = reg.interner_mut().intern("Morgue Burst");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card from your graveyard to your hand. Morgue Burst deals damage to any target equal to the power of the card returned this way.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::any_target(),
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
    use arcana_core::targets::TargetChoice;
    // GAP: damage amount = power of returned creature card (runtime lookup)
    let Some(TargetChoice::Object(graveyard_id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *graveyard_id }]
}
