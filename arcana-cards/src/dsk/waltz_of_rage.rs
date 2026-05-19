//! Waltz of Rage — `{3}{R}{R}` sorcery, "Target creature you control deals damage equal
//! to its power to each other creature. Until end of turn, whenever a creature you control
//! dies, exile the top card of your library. You may play it until the end of your next turn."
//!
//! GAP: "whenever a creature you control dies this turn, exile top card and may play it"
//! is a turn-lasting triggered ability that cannot be attached to the player via available
//! Effect variants.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waltz of Rage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to each other creature. Until end of turn, whenever a creature you control dies, exile the top card of your library. You may play it until the end of your next turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(source_id) = target else { return Vec::new(); };
    let power = script::power_of(state, *source_id);
    if power <= 0 {
        // GAP: turn-lasting triggered ability on player not expressible
        return Vec::new();
    }
    let all_creatures = script::ids_matching(state, &arcana_core::targets::ObjectFilter::creature(), entry.controller);
    let others: Vec<_> = all_creatures.into_iter().filter(|&id| id != *source_id).collect();
    // GAP: turn-lasting triggered ability ("whenever a creature you control dies this turn") not expressible
    vec![Effect::ForEach {
        targets: others,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: power as u32,
        }),
    }]
}
