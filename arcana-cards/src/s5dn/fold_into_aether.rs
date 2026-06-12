//! Fold into Aether — `{2}{U}{U}` instant. "Counter target spell. If
//! that spell is countered this way, its controller may put a
//! creature card from their hand onto the battlefield." The rider is
//! an optional PutFromHandOntoBattlefield pick made by the countered
//! spell's controller.
//! GAP: "if that spell is countered this way" — the rider is emitted
//! unconditionally; no hook conditions on counter success (over-fires
//! only against can't-be-countered spells).

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fold into Aether");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. If that spell is countered this way, its controller may put a creature card from their hand onto the battlefield.".into(),
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::Counter { target: *id }];
    // "its controller may put a creature card from their hand onto the
    // battlefield" — optional pick by the countered spell's controller.
    // GAP: emitted unconditionally; see module doc.
    if let Some(spell_controller) = state.objects.get(*id).map(|o| o.controller) {
        effects.push(Effect::PutFromHandOntoBattlefield {
            player: spell_controller,
            filter: ObjectFilter::creature(),
            tapped: false,
        });
    }
    effects
}
