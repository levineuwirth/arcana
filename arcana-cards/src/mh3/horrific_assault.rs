//! Horrific Assault — `{G}` sorcery. Target creature you control deals
//! damage = its power to target creature/planeswalker you don't control.
//! If you control an Eldrazi, gain 3 life.

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
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horrific Assault");
    let _ = reg.interner_mut().intern("Eldrazi");
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
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))
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
    let Some(a) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(b) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(a_id) = a else { return Vec::new(); };
    let TargetChoice::Object(b_id) = b else { return Vec::new(); };
    // Use Fight as the closest one-sided proxy (catalog only has two-sided
    // fight). GAP: real one-sided "deals damage equal to its power" not in
    // catalog.
    let mut effects: Vec<Effect> = vec![Effect::Fight { a: *a_id, b: *b_id }];
    let has_eldrazi = script::count_matching(
        state,
        &script::subtype_filter(reg, "Eldrazi").controlled_by(ControllerConstraint::You),
        entry.controller,
    ) > 0;
    if has_eldrazi {
        effects.push(Effect::GainLife { player: entry.controller, amount: 3 });
    }
    effects
}
