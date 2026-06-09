//! Malamet Battle Glyph — `{G}` sorcery. "Choose target creature you
//! control and target creature you don't control. If the creature
//! you control entered this turn, put a +1/+1 counter on it. Then
//! those creatures fight each other." The counter is gated on the
//! per-id `script::entered_battlefield_this_turn` check.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malamet Battle Glyph");
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
                text: "Choose target creature you control and target creature you don't control. If the creature you control entered this turn, put a +1/+1 counter on it. Then those creatures fight each other.".into(),
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(t0) = entry.targets.targets.get(0) else { return Vec::new(); };
    let Some(t1) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(a) = t0 else { return Vec::new(); };
    let TargetChoice::Object(b) = t1 else { return Vec::new(); };
    let mut effects = Vec::new();
    // "If the creature you control entered this turn, put a +1/+1 counter on it."
    if script::entered_battlefield_this_turn(state, *a) {
        effects.push(Effect::AddCounters {
            target: *a,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects.push(Effect::Fight { a: *a, b: *b });
    effects
}
