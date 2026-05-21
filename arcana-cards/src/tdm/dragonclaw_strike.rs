//! Dragonclaw Strike — `{2/G}{2/U}{2/R}` sorcery. "Double the power
//! and toughness of target creature you control until end of turn.
//! Then it fights up to one target creature an opponent controls."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Dragonclaw Strike");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2/G}{2/U}{2/R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Double the power and toughness of target creature you control until end of turn. Then it fights up to one target creature an opponent controls.".into(),
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

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut iter = entry.targets.targets.iter();
    let Some(src_t) = iter.next() else { return Vec::new(); };
    let TargetChoice::Object(src_id) = src_t else { return Vec::new(); };
    let p = script::power_of(state, *src_id);
    let t = script::toughness_of(state, *src_id);
    let mut effects = vec![Effect::Pump {
        target: *src_id,
        power: p,
        toughness: t,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(opp_t) = iter.next() {
        if let TargetChoice::Object(opp_id) = opp_t {
            effects.push(Effect::Fight { a: *src_id, b: *opp_id });
        }
    }
    effects
}
