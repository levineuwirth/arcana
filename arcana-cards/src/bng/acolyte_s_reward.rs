//! Acolyte's Reward — `{1}{W}` instant. "Prevent the next X damage that
//! would be dealt to target creature this turn, where X is your devotion
//! to white. If damage is prevented this way, Acolyte's Reward deals that
//! much damage to any target."
//!
//! The first clause is expressible: a `PreventDamage` on the targeted
//! creature with the amount computed via `script::devotion(.., white)`.
//! The second clause ("if damage is prevented this way, deal that much
//! damage to any target") requires a delayed/triggered amount equal to the
//! quantity actually prevented and a second target chosen on resolution —
//! there is no primitive to thread the prevented quantity into a later
//! `DealDamage`. That rider is GAPped; the prevention is still emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Acolyte's Reward");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Prevent the next X damage that would be dealt to target \
                       creature this turn, where X is your devotion to white. If \
                       damage is prevented this way, Acolyte's Reward deals that much \
                       damage to any target."
                    .into(),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = script::devotion(state, entry.controller, ColorSet::white());
    // GAP: the "if damage is prevented this way, deal that much damage to any
    // target" rider cannot be expressed — no primitive threads the actually
    // prevented quantity into a later DealDamage against a second target.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: Some(x),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
