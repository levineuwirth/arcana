//! Return to Dust — `{2}{W}{W}` instant. "Exile target artifact or
//! enchantment. If you cast this spell during your main phase, you may
//! exile up to one other target artifact or enchantment." We model the
//! optional second target as a second TargetRequirement (engine offers it as
//! up-to-one). No phase-check helper — GAP the main-phase condition (we
//! always allow the second exile).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

fn artifact_or_enchantment_filter() -> TargetFilter {
    TargetFilter::Permanent(
        ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .with_types_any(TypeLine::ENCHANTMENT.into()),
    )
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Return to Dust");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target artifact or enchantment. If you cast this spell during your main phase, you may exile up to one other target artifact or enchantment.".into(),
            // GAP: no helper for "cast during your main phase" gating; second target is always offered.
            target_requirements: vec![
                TargetRequirement {
                    filter: artifact_or_enchantment_filter(),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: artifact_or_enchantment_filter(),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = Vec::new();
    for t in entry.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            out.push(Effect::ExilePermanent { target: *id });
        }
    }
    out
}
