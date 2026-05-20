//! Betrayal at the Vault — `{4}{G}{G}` instant, "Target creature you
//! control deals damage equal to its power to each of two other target
//! creatures."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Betrayal at the Vault");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its \
                   power to each of two other target creatures."
                .into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
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
    let ts = &entry.targets.targets;
    let (Some(TargetChoice::Object(src)), Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) =
        (ts.first(), ts.get(1), ts.get(2))
    else {
        return Vec::new();
    };
    let pw = script::power_of(state, *src).max(0) as u32;
    vec![
        Effect::DealDamage {
            source: *src,
            target: DamageTarget::Object(*a),
            amount: pw,
        },
        Effect::DealDamage {
            source: *src,
            target: DamageTarget::Object(*b),
            amount: pw,
        },
    ]
}
