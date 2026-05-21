//! Finishing Move — `{2}{G}` sorcery. "You get {TK}{TK}, then you may put
//! a sticker on a nonland permanent you own. Target creature you control
//! deals damage equal to its power to target creature you don't control."
//! The ticket/sticker mechanic is not expressible; only the fight-style
//! damage is emitted.

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
    let name = reg.interner_mut().intern("Finishing Move");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You get {TK}{TK}, then you may put a sticker on a nonland permanent you own. Target creature you control deals damage equal to its power to target creature you don't control.".into(),
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

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: ticket / sticker mechanic ({TK}{TK} and putting a sticker) is unmodeled.
    let Some(TargetChoice::Object(source)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(victim)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    let amount = script::power_of(state, *source).max(0) as u32;
    vec![Effect::DealDamage {
        source: *source,
        target: DamageTarget::Object(*victim),
        amount,
    }]
}
