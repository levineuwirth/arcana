//! Roiling Terrain — `{2}{R}{R}` sorcery. "Destroy target land, then
//! Roiling Terrain deals damage to that land's controller equal to
//! the number of land cards in that player's graveyard."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roiling Terrain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target land, then Roiling Terrain deals damage to that land's controller equal to the number of land cards in that player's graveyard.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::LAND.into()),
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
    let controller = script::target_controller(state, *id, entry.controller);
    let lands_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let amount = script::graveyard_matching(state, &lands_filter, controller, entry.controller);
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(controller),
            amount,
        },
    ]
}
