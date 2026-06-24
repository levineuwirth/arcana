//! Pistus Strike — `{2}{G}` instant. "Destroy target creature with
//! flying. Its controller gets a poison counter." The with-flying
//! target restriction is enforced via
//! `ObjectFilter::creature().with_keyword(Flying)`. The poison counter
//! goes on the targeted creature's controller via
//! Effect::GivePlayerCounters { kind: Poison } (controller read pre-
//! destruction via script::target_controller).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pistus Strike");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature with flying. Its controller gets a poison counter.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                ),
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
    // "Its controller gets a poison counter." Read the controller before the
    // destroy effect applies (script::target_controller reads current state).
    let controller = script::target_controller(state, *id, entry.controller);
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::GivePlayerCounters { player: controller, kind: CounterKind::Poison, count: 1 },
    ]
}
