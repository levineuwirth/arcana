//! Burrog Barrage — `{1}{G}` instant, "Target creature you control
//! gets +1/+0 until end of turn if you've cast another instant or
//! sorcery spell this turn. Then it deals damage equal to its power
//! to up to one target creature an opponent controls."
//!
//! The "deals damage equal to its power" part is applied. GAP: "if
//! you've cast another instant or sorcery this turn" cast-history
//! condition is not scriptable, so the conditional +1/+0 is omitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burrog Barrage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gets +1/+0 until end of turn if you've cast another instant or sorcery spell this turn. Then it deals damage equal to its power to up to one target creature an opponent controls.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: conditional +1/+0 (cast-history condition) omitted.
    let mut ts = entry.targets.targets.iter();
    let Some(TargetChoice::Object(src)) = ts.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(victim)) = ts.next() else { return Vec::new(); };
    let amount = script::power_of(state, *src).max(0) as u32;
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*victim),
        amount,
    }]
}
