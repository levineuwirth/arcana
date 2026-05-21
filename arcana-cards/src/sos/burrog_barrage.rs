//! Burrog Barrage — `{1}{G}` instant. "Target creature you control gets +1/+0
//! until end of turn if you've cast another instant or sorcery spell this
//! turn. Then it deals damage equal to its power to up to one target creature
//! an opponent controls."
//!
//! 'Cast another instant or sorcery this turn' history flag isn't in script::.
//! GAP the conditional pump; emit the fight half (damage equal to power).

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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
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
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
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

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = &entry.targets.targets;
    let Some(t0) = targets.first() else { return Vec::new(); };
    let TargetChoice::Object(a) = t0 else { return Vec::new(); };
    // GAP: 'have cast another instant/sorcery this turn' history flag → omit pump.
    let mut effects: Vec<Effect> = Vec::new();
    if let Some(TargetChoice::Object(b)) = targets.get(1) {
        let dmg = script::power_of(state, *a).max(0) as u32;
        effects.push(Effect::DealDamage {
            source: *a,
            target: DamageTarget::Object(*b),
            amount: dmg,
        });
    }
    effects
}
