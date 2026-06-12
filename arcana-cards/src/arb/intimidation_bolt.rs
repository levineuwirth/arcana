//! Intimidation Bolt — `{1}{R}{W}` instant. "Intimidation Bolt deals
//! 3 damage to target creature. Other creatures can't attack this
//! turn." The attack restriction is a filtered can't-attack static
//! until end of turn. GAP (narrowed): no exclude-by-id filter, so the
//! targeted creature is also restricted (oracle exempts it).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Intimidation Bolt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Intimidation Bolt deals 3 damage to target creature. Other creatures can't attack this turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 3,
        },
        // "Other creatures can't attack this turn." GAP (narrowed): no
        // exclude-by-id filter — the targeted creature is restricted too.
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_cant_attack(
                entry.source,
                ObjectFilter::creature(),
                Duration::EndOfTurn,
            ),
        },
    ]
}
