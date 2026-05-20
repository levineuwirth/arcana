//! Fading Hope — `{U}` instant. "Return target creature to its
//! owner's hand. If its mana value was 3 or less, scry 1."
//!
//! "If its mana value was 3 or less" post-resolution lookup is not
//! in catalog Conditional; best-effort: bounce + unconditional scry.
//! (The conservative shape is just bounce — scry-unconditional is a
//! soft mismatch we mark as a GAP comment.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fading Hope");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature to its owner's hand. If its mana value was 3 or less, scry 1.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "if its mana value was 3 or less" conditional not in catalog. Only the bounce is modeled.
    vec![Effect::ReturnToHand { target: *id }]
}
