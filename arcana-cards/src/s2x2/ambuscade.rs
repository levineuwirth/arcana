//! Ambuscade — `{2}{G}` instant, "Target creature you control gets +1/+0
//! until end of turn. Then it deals damage equal to its power to target
//! creature an opponent controls."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ambuscade");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 until end of turn. Then it deals damage equal to its power to target creature an opponent controls.".into(),
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
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(t1) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(id_a) = t0 else { return Vec::new(); };
    let TargetChoice::Object(id_b) = t1 else { return Vec::new(); };
    // Pump applies first, then read power for the damage
    let power_after_pump = script::power_of(state, *id_a) + 1;
    vec![
        Effect::Pump {
            target: *id_a,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id_b),
            amount: power_after_pump.max(0) as u32,
        },
    ]
}
