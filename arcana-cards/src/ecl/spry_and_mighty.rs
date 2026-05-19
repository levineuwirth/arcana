//! Spry and Mighty — `{4}{G}` sorcery.
//! "Choose exactly two creatures you control. You draw X cards and the chosen
//! creatures get +X/+X and gain trample until end of turn, where X is the
//! difference between the chosen creatures' powers."
//! GAP: No engine support for 'choose exactly two creatures you control';
//! target_requirements and multi-target pump with a computed delta require
//! two explicit creature targets and power_of reads.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spry and Mighty");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose exactly two creatures you control. You draw X cards and the chosen creatures get +X/+X and gain trample until end of turn, where X is the difference between the chosen creatures' powers.".into(),
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
                                .controlled_by(ControllerConstraint::You),
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
    let Some(TargetChoice::Object(id_a)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(id_b)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    let pow_a = script::power_of(state, *id_a);
    let pow_b = script::power_of(state, *id_b);
    let x = (pow_a - pow_b).abs();
    let xu = x as u32;
    let mut effects = Vec::new();
    effects.push(Effect::DrawCards { player: entry.controller, count: xu });
    effects.push(Effect::Pump {
        target: *id_a,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    });
    effects.push(Effect::Pump {
        target: *id_b,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    });
    effects
}
