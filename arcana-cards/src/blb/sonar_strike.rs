//! Sonar Strike — `{1}{W}` instant. "Sonar Strike deals 4 damage to
//! target attacking, blocking, or tapped creature. You gain 3 life if
//! you control a Bat."
//!
//! 4 damage to the targeted creature. The "you gain 3 life if you
//! control a Bat" rider is wired via a subtype_filter count.
//!
//! GAP: "attacking, blocking, or tapped" target restriction is only
//! partially expressible (tapped via tapped_only, but not
//! attacking/blocking); the target is left unrestricted to avoid
//! over-narrowing.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sonar Strike");
    let _bat = reg.interner_mut().intern("Bat");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Sonar Strike deals 4 damage to target attacking, blocking, or tapped creature. You gain 3 life if you control a Bat.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "attacking/blocking/tapped" target restriction not fully expressible.
    let mut effects = vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }];
    let has_bat = script::count_matching(
        state,
        &script::subtype_filter(reg, "Bat").controlled_by(ControllerConstraint::You),
        entry.controller,
    ) > 0;
    if has_bat {
        effects.push(Effect::GainLife { player: entry.controller, amount: 3 });
    }
    effects
}
