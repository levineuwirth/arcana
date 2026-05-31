//! Foray of Orcs — `{3}{R}` sorcery. "Amass Orcs 2. When you do, Foray of
//! Orcs deals X damage to target creature an opponent controls, where X is the
//! amassed Army's power."
//!
//! Implemented with [`Effect::Amass`] (grow/create a black Orc Army with two
//! +1/+1 counters) followed by damage to the targeted opponent creature. The
//! amassed Army's power X is computed before the amass: the lowest-id Army the
//! controller already controls (the engine's deterministic amass target) plus
//! the 2 counters about to be added; if no Army exists yet, the freshly minted
//! 0/0 Army becomes 2/2, so X = 2.

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
    let name = reg.interner_mut().intern("Foray of Orcs");
    let _army = reg.interner_mut().intern("Army");
    let _orc = reg.interner_mut().intern("Orc");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Amass Orcs 2. When you do, Foray of Orcs deals X damage to \
                   target creature an opponent controls, where X is the \
                   amassed Army's power."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let army_sub = reg
        .interner()
        .lookup("Army")
        .expect("Army interned during register()");
    let orc_sub = reg
        .interner()
        .lookup("Orc")
        .expect("Orc interned during register()");

    // X = the amassed Army's power. The engine grows the lowest-id Army the
    // controller already controls; its power becomes (current power + 2). With
    // no existing Army, a fresh 0/0 Army is minted and becomes 2/2 → X = 2.
    let mut armies = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_subtypes_any(vec![army_sub]),
        entry.controller,
    );
    armies.sort();
    let base = armies
        .first()
        .map(|id| script::power_of(state, *id))
        .unwrap_or(0);
    let x = (base + 2).max(0) as u32;

    let mut effects = vec![Effect::Amass {
        controller: entry.controller,
        count: 2,
        army_subtype: army_sub,
        race_subtype: orc_sub,
    }];

    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: x,
        });
    }
    effects
}
