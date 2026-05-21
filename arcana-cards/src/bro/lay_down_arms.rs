//! Lay Down Arms — `{W}` sorcery. "Exile target creature with mana
//! value less than or equal to the number of Plains you control. Its
//! controller gains 3 life." We approximate the bound using
//! count_matching on Plains-named lands — but ObjectFilter can't
//! filter by basic-land subtype without a subtype helper. The exile
//! target's mana-value bound is also DYNAMIC, so we GAP rather than
//! emit a wrong-MV-bound target requirement.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lay Down Arms");
    let _plains = reg.interner_mut().intern("Plains");
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
                // GAP: target mana-value bound is dynamic on number of
                // Plains; target_requirements can't express that.
                text: "Exile target creature with mana value less than or equal to the number of Plains you control. Its controller gains 3 life.".into(),
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let owner = script::target_controller(state, *id, entry.controller);
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::GainLife { player: owner, amount: 3 },
    ]
}
