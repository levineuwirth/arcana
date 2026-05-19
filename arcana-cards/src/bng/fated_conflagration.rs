//! Fated Conflagration — `{1}{R}{R}{R}` instant. "Fated Conflagration deals 5 damage to target
//! creature or planeswalker. If it's your turn, scry 2."
//! GAP: "if it's your turn" conditional around scry is not expressible with catalog.
//! Best effort: deal 5 damage; scry is omitted.

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
    let name = reg.interner_mut().intern("Fated Conflagration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Fated Conflagration deals 5 damage to target creature or planeswalker. If it's your turn, scry 2.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .with_types_any(TypeLine::CREATURE.into()),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            arcana_core::targets::ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            arcana_core::targets::ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    // GAP: "if it's your turn, scry 2" — turn-ownership conditional on scry not in catalog
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 5,
    }]
}
