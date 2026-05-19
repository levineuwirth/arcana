//! Devouring Tendrils — `{1}{G}` sorcery, "Target creature you control deals damage equal to
//! its power to target creature or planeswalker you don't control. When the permanent you don't
//! control dies this turn, you gain 2 life."
//!
//! GAP: 'target creature or planeswalker' — TargetFilter has no Planeswalker variant; using
//! Creature target for the non-controller permanent. The 'when it dies this turn, gain 2 life'
//! triggered ability cannot be registered from a spell resolver.

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
    let name = reg.interner_mut().intern("Devouring Tendrils");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature or planeswalker you don't control. When the permanent you don't control dies this turn, you gain 2 life.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
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
    let Some(attacker_target) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(defender_target) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(attacker_id) = attacker_target else { return Vec::new(); };
    let TargetChoice::Object(defender_id) = defender_target else { return Vec::new(); };
    let _pwr = script::power_of(state, *attacker_id);
    // GAP: one-directional 'deals damage equal to its power' (Fight is symmetric); 'when defender
    //      dies this turn, gain 2 life' triggered ability from resolver; planeswalker target not
    //      available in TargetFilter
    vec![Effect::Fight { a: *attacker_id, b: *defender_id }]
}
