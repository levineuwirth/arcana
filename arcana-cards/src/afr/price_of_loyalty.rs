//! Price of Loyalty — `{2}{R}` sorcery, "Gain control of target creature
//! until end of turn. Untap that creature. It gains haste until end of turn.
//! If mana from a Treasure was spent to cast this spell, that creature gets
//! +2/+0 until end of turn."
//!
//! GAP: "gain control of target creature until end of turn" — control-change
//! effect is not in the Effect catalog.
//! GAP: "if mana from a Treasure was spent" — mana source tracking not
//! available.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Price of Loyalty");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Gain control of target creature until end of turn. Untap that creature. It gains haste until end of turn. If mana from a Treasure was spent to cast this spell, that creature gets +2/+0 until end of turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: control change until end of turn not in Effect catalog.
    // GAP: Treasure mana source tracking not available.
    // Partial: Untap is expressible.
    vec![Effect::Untap { target: *id }]
}
