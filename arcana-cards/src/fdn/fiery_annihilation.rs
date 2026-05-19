//! Fiery Annihilation — `{2}{R}` instant.
//! "Fiery Annihilation deals 5 damage to target creature. Exile up to one
//! target Equipment attached to that creature. If that creature would die
//! this turn, exile it instead."
//!
//! # GAP: 'exile instead of die this turn' replacement effect — no Effect
//! variant models a death-replacement for the remainder of the turn.
//! # GAP: Equipment subtype filter for ExilePermanent — ObjectFilter has no
//! subtype predicate; modelled as generic artifact exile.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fiery Annihilation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Fiery Annihilation deals 5 damage to target creature. Exile up to one target Equipment attached to that creature. If that creature would die this turn, exile it instead.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile-instead-of-die replacement effect for the turn
    let mut effects = Vec::new();
    let mut iter = entry.targets.targets.iter();
    if let Some(TargetChoice::Object(creature_id)) = iter.next() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*creature_id),
            amount: 5,
        });
    }
    if let Some(TargetChoice::Object(equip_id)) = iter.next() {
        effects.push(Effect::ExilePermanent { target: *equip_id });
    }
    effects
}
