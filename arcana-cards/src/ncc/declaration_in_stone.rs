//! Declaration in Stone — `{1}{W}` sorcery. "Exile target creature and all
//! other creatures its controller controls with the same name as that creature.
//! That player investigates for each nontoken creature exiled this way."
//!
//! The exile sweep is "the target creature plus all OTHER creatures its
//! controller controls SHARING THE TARGET'S NAME" — a same-name-as-the-target
//! mass exile that cannot be enumerated from the `script::` helpers (no
//! name-equality-to-a-chosen-permanent filter), and the Investigate count is
//! "for each NONTOKEN creature exiled this way" (dynamic on the size of that
//! sweep). Emitting a literal Clue count or only the same-name subset would be
//! a materially wrong card. Best-effort: exile the single chosen target; GAP
//! the same-name mass exile and the dynamic per-exiled-creature Investigate.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Declaration in Stone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature and all other creatures its controller controls with the same name as that creature. That player investigates for each nontoken creature exiled this way.".into(),
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
    // GAP: cannot enumerate "all OTHER creatures its controller controls with
    // the same name as the target" (no name-equality-to-a-chosen-permanent
    // filter), and the Investigate count is "for each nontoken creature exiled
    // this way" — dynamic on that un-enumerable sweep.
    vec![Effect::ExilePermanent { target: *id }]
}
