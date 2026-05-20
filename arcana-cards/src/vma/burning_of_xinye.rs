//! Burning of Xinye — `{4}{R}{R}` sorcery. "You destroy four lands
//! you control, then target opponent destroys four lands they
//! control. Then Burning of Xinye deals 4 damage to each creature."
//! Modeled as: you sacrifice 4 lands, the target opponent sacrifices
//! 4 lands, then 4 damage to every creature.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burning of Xinye");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You destroy four lands you control, then target opponent destroys four lands they control. Then Burning of Xinye deals 4 damage to each creature.".into(),
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
    let Some(TargetChoice::Player(opp)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let land = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    );
    vec![
        Effect::Sacrifice {
            player: entry.controller,
            filter: land.clone(),
            count: 4,
        },
        Effect::Sacrifice {
            player: *opp,
            filter: land,
            count: 4,
        },
        Effect::ForEach {
            targets: creatures,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 4,
            }),
        },
    ]
}
