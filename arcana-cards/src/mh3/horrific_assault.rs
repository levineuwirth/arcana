//! Horrific Assault — `{G}` sorcery, "Target creature you control deals
//! damage equal to its power to target creature or planeswalker you
//! don't control. If you control an Eldrazi, you gain 3 life."

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
    let name = reg.interner_mut().intern("Horrific Assault");
    let _eldrazi = reg.interner_mut().intern("Eldrazi");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature or planeswalker you don't control. If you control an Eldrazi, you gain 3 life.".into(),
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

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut iter = entry.targets.targets.iter();
    let Some(TargetChoice::Object(src)) = iter.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(tgt)) = iter.next() else { return Vec::new(); };
    let power = script::power_of(state, *src).max(0) as u32;
    let mut effects = vec![Effect::DealDamage {
        source: *src,
        target: DamageTarget::Object(*tgt),
        amount: power,
    }];
    let eldrazi = script::count_matching(
        state,
        &script::subtype_filter(reg, "Eldrazi")
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if eldrazi > 0 {
        effects.push(Effect::GainLife { player: entry.controller, amount: 3 });
    }
    effects
}
