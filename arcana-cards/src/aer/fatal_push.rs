//! Fatal Push — `{B}` instant. "Destroy target creature if it has mana value 2
//! or less."
//!
//! Revolt (CR 702.116) WIRED: the threshold rises to mana value 4 if a permanent
//! you controlled left the battlefield this turn, via the new
//! `script::permanent_left_battlefield_this_turn` event-history accessor
//! (LeavesBattlefield events in the per-turn slice, controller read from LKI).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fatal Push");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature if it has mana value 2 or less."
                .into(),
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
    let id = match entry.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let Some(obj) = state.objects.get(id) else { return Vec::new(); };
    // Revolt (CR 702.116): threshold rises to MV 4 if a permanent you controlled
    // left the battlefield this turn, else MV 2.
    let threshold = if arcana_core::script::permanent_left_battlefield_this_turn(state, entry.controller) {
        4
    } else {
        2
    };
    if obj.characteristics.mana_value() <= threshold {
        vec![Effect::DestroyPermanent { target: id }]
    } else {
        Vec::new()
    }
}
