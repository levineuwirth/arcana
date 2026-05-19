//! Sage's Dousing — `{2}{U}` Kindred Instant — Wizard. "Counter target spell unless its
//! controller pays {3}. If you control a Wizard, draw a card."
//!
//! Note: TypeLine does not have a KINDRED constant; using INSTANT only as closest match.
//! GAP: Kindred type not in TypeLine constants; conditional draw based on controlling a subtype
//! requires script::subtype_filter but 'if you control X' conditional not expressible inline.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sage's Dousing");
    let _wizard = reg.interner_mut().intern("Wizard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell unless its controller pays {3}. If you control a Wizard, draw a card.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let wizard_count = script::count_matching(
        state,
        &script::subtype_filter(reg, "Wizard").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects = vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{3}").expect("valid cost"),
    }];
    if wizard_count > 0 {
        effects.push(Effect::DrawCards { player: entry.controller, count: 1 });
    }
    effects
}
