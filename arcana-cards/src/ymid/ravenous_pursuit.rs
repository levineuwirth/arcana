//! Ravenous Pursuit — `{1}{G}` sorcery. "Target creature you control
//! deals damage equal to its power to target creature you don't
//! control. Choose a creature card in your hand. It perpetually gets
//! +X/+X, where X is the amount of excess damage dealt this way."

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
    let name = reg.interner_mut().intern("Ravenous Pursuit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its power to target creature you don't control. Choose a creature card in your hand. It perpetually gets +X/+X, where X is the amount of excess damage dealt this way.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let (Some(TargetChoice::Object(src)), Some(TargetChoice::Object(tgt))) = (
        entry.targets.targets.first(),
        entry.targets.targets.get(1),
    ) else {
        return Vec::new();
    };
    let amount = script::power_of(state, *src).max(0) as u32;
    // GAP: the "choose a creature in your hand, perpetually +X/+X by
    // excess damage" rider is not expressible; only the source creature's
    // damage to the target is emitted.
    vec![Effect::DealDamage {
        source: *src,
        target: DamageTarget::Object(*tgt),
        amount,
    }]
}
