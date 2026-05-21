//! Ghostly Flicker — `{2}{U}` instant. "Exile two target artifacts,
//! creatures, and/or lands you control, then return those cards to the
//! battlefield under your control."

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghostly Flicker");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let acl_filter = || ObjectFilter::permanent()
        .with_types_any(TypeLine(
            TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::LAND,
        ))
        .controlled_by(ControllerConstraint::You);
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile two target artifacts, creatures, and/or lands you control, then return those cards to the battlefield under your control.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(acl_filter()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(acl_filter()),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out: Vec<Effect> = Vec::new();
    for t in entry.targets.targets.iter().take(2) {
        if let TargetChoice::Object(id) = t {
            out.push(Effect::ExilePermanent { target: *id });
            out.push(Effect::DelayedAction {
                source: *id,
                controller: entry.controller,
                when: DelayedWhen::NextEndStep,
                action: DelayedAction::ReturnFromExileToBattlefield,
            });
        }
    }
    out
}
