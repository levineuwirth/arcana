//! Lay Down Arms — `{W}` sorcery. "Exile target creature with mana value
//! less than or equal to the number of Plains you control. Its
//! controller gains 3 life."
//!
//! GAP: ManaValueConditional (filtering targets by mana value ≤ count
//! of Plains you control) is not expressible in TargetFilter. Using
//! plain creature target; the Plains-count constraint and life gain for
//! the target's controller are noted as gaps.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lay Down Arms");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature with mana value less than or equal to the number of Plains you control. Its controller gains 3 life.".into(),
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
    // GAP: ManaValueConditional (target restricted to MV ≤ number of Plains you control)
    // GAP: TargetControllerGainLife (target creature's controller gains 3 life)
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: *id }]
}
