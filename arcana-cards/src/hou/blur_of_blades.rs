//! Blur of Blades — `{1}{R}` instant. "Put a -1/-1 counter on target
//! creature. Blur of Blades deals 2 damage to that creature's
//! controller." GAP: -1/-1 counter kind not in catalog (only
//! PlusOnePlusOne). Also GAP target's-controller damage rider — we
//! still express the catalog-mappable piece by GAPping the whole
//! effect, since pumping with a wrong counter kind is materially
//! wrong.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blur of Blades");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a -1/-1 counter on target creature. Blur of Blades deals 2 damage to that creature's controller.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: -1/-1 counter kind not in catalog (only PlusOnePlusOne is exposed).
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let controller = script::target_controller(state, *id, entry.controller);
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(controller),
        amount: 2,
    }]
}
